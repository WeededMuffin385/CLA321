docker compose exec gitlab-runner \
  gitlab-runner register \
    --non-interactive \
    --url "http://gitlab" \
    --token "$RUNNER_TOKEN" \
    --executor "docker" \
    --docker-image "alpine:latest"