

### Settings Example

```yaml

envs:
  env2:
    url: "ssh:test@10.0.0.0:22->http://localhost:5001"
    users: Group2

  env1:
    url: "http://localhost:5000"
    users: Group1

ssh_private_keys:
  ssh:test@10.0.0.0:22:
    cert_path: ~/.ssh/id_rsa
    cert_pass_phrase: pass

users:
  Group1:
  - User1
  - User2
  Group2:
  - User3
  - User4

```

WildCards are supported

```yaml

envs:
  env2:
    url: "ssh:test@10.0.0.0:22->http://localhost:5001"
    users: "*"

```