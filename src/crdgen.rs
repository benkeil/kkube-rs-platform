use kube::CustomResourceExt;

mod controllers;
mod lib;
use controllers::aws_account::spec::AwsAccount;

fn main() {
    print!("{}", serde_yaml::to_string(&AwsAccount::crd()).unwrap())
}
