extern crate platform;
use kube::CustomResourceExt;

use platform::aws_account::spec::AwsAccount;

fn main() {
    print!("{}", serde_yaml::to_string(&AwsAccount::crd()).unwrap())
}
