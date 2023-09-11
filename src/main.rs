use futures::{StreamExt, TryStreamExt};
use kube::ResourceExt;
use log::info;
use log4rs;
mod controllers;
use controllers::aws_account::controller::{self, State};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("start controller");

    // Infer the runtime environment and try to create a Kubernetes Client
    // let client = Client::try_default().await?;

    let state = State::default();
    let controller = controller::run(state.clone());
    tokio::join!(controller).0;

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    // info!("pods on startup");
    // let pods: Api<Pod> = Api::namespaced(client, "default");
    // for p in pods.list(&ListParams::default()).await? {
    //     info!("found pod {}", p.name_any());
    //     info!("labels: {:?}", p.labels());
    // }
    //
    // info!("watch for pods");
    // let wc = watcher::Config::default();
    // let mut apply_stream = watcher(pods, wc).boxed();
    // while let Some(event) = apply_stream.try_next().await? {
    //     match event {
    //         Event::Applied(pod) => {
    //             info!("applied {}", pod.name_any());
    //         }
    //         Event::Deleted(pod) => {
    //             info!("deleted {}", pod.name_any());
    //         }
    //         _ => {}
    //     }
    // }

    Ok(())
}
