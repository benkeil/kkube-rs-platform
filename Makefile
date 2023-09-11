kind-create:
	kind create cluster

kind-delete:
	kind delete cluster


kubectl-apply:
	kubectl apply -k demo/hello-world

kubectl-delete:
	kubectl delete -k demo/hello-world


crd-generate:
	cargo run --bin crdgen > crd/aws-account.crd.yaml

crd-install:
	kubectl apply -f crd/aws-account.crd.yaml

crd-delete:
	kubectl apply -f crd/aws-account.crd.yaml


aws-account-apply:
	kubectl apply -f demo/aws-account

aws-account-delete:
	kubectl delete -f demo/aws-account
