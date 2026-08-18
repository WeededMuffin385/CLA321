# Hello world

Get initial gitlab password:

```bash
(
    mkdir -p assets
    touch assets/gitlab_password.txt
    (
    
      docker compose exec gitlab \
      grep 'Password:' /etc/gitlab/initial_root_password
    ) > assets/gitlab_password.txt
)
```
