
build-docker SERVICE_PATH:
    nix build ./services/{{SERVICE_PATH}}#dockerImage

load-docker SERVICE_PATH: (build-docker SERVICE_PATH)
    ./result | docker load

load-minikube SERVICE: (build-docker SERVICE)
    minikube image rm rusty-graph-{{SERVICE}}:latest
    ./result | minikube image load -

deploy:
    kubectl delete --ignore-not-found=true -k manifests/
    for service in `ls services/`; do just load-minikube $service; done
    kubectl apply -k manifests/nginx-gateway
    kubectl apply -k manifests/