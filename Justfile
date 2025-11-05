# Deploy to production (build image, push, restart)
deploy:
    docker build . --tag code.bearcove.cloud/bearcove/bearcove-eu:arm64
    docker push code.bearcove.cloud/bearcove/bearcove-eu:arm64
    kubectl rollout restart deployment/bearcove-eu -n bearcove-eu
