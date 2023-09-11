use std::sync::Arc;
use std::time::Duration;

use crate::controllers::aws_account::spec::{AwsAccount, CRD_NAME};
use futures::StreamExt;
use kube::api::ListParams;
use kube::runtime::controller::Action;
use kube::runtime::watcher::Config;
use kube::runtime::{finalizer, Controller};
use kube::{Api, Client, ResourceExt};
use log::{error, info, warn};
use platform::{Error, Result};

/// Initialize the controller and shared state (given the crd is installed)
pub async fn run(state: State) {
    let client = Client::try_default()
        .await
        .expect("failed to create kube Client");
    let aws_account_api = Api::<AwsAccount>::all(client.clone());
    if let Err(e) = aws_account_api.list(&ListParams::default().limit(1)).await {
        error!("CRD is not queryable; {e:?}. Is the CRD installed?");
        info!("Installation: cargo run --bin crdgen | kubectl apply -f -");
        std::process::exit(1);
    }

    Controller::new(aws_account_api, Config::default().any_semantic())
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(Context { client }))
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("reconciled {:?}", o),
                Err(e) => warn!("reconcile failed: {}", e),
            }
        })
        .await;
}

async fn reconcile(aws_account: Arc<AwsAccount>, ctx: Arc<Context>) -> Result<Action> {
    let namespace = aws_account.namespace().unwrap(); // aws account queue is namespace scoped
    let aws_account_api: Api<AwsAccount> = Api::namespaced(ctx.client.clone(), &namespace);

    info!(
        "Reconciling {} \"{}\" in {}",
        CRD_NAME,
        aws_account.name_any(),
        namespace
    );

    finalizer(&aws_account_api, CRD_NAME, aws_account, |event| async {
        match event {
            finalizer::Event::Apply(result) => result.reconcile(ctx.clone()).await,
            finalizer::Event::Cleanup(result) => result.cleanup(ctx.clone()).await,
        }
    })
    .await
    .map_err(|e| Error::FinalizerError(Box::new(e)))
}

fn error_policy(aws_account: Arc<AwsAccount>, error: &Error, ctx: Arc<Context>) -> Action {
    warn!("reconcile failed: {:?}", error);
    Action::requeue(Duration::from_secs(5))
}

impl AwsAccount {
    // Reconcile (for non-finalizer related changes)
    async fn reconcile(&self, ctx: Arc<Context>) -> Result<Action> {
        Ok(Action::requeue(Duration::from_secs(10)))
    }

    // Finalizer cleanup (the object was deleted, ensure nothing is orphaned)
    async fn cleanup(&self, ctx: Arc<Context>) -> Result<Action> {
        Ok(Action::await_change())
    }
}

// Context for our reconciler
#[derive(Clone)]
pub struct Context {
    /// Kubernetes client
    pub client: Client,
}

/// State shared between the controller and the web server
#[derive(Clone, Default)]
pub struct State {
    pub foo: String,
}
