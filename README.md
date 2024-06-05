# Renchmark

Rust-based multi-node infrastructure and architecture benchmark utility

- **Multi-node:** Orchestrate your benchmark
- **infrastructure:** Check your servers metrics
- **Architecture:** Check your API stress load

All by using `YAML` files

```shell
renchmark bench.yaml
```

## Config

Proposed yaml file

```yaml
renchmark:
  generate_file: $date.json
  generate_html: true
  show_summary: true
  users:
    total: 100000
    concurrent: 1000
    speed: 1s # without the actions will occur one right after each other.
    fail:
      less_than: 250
      after: 120m
  app_nodes:
  auth:
    type: SSH
    is_docker: false
  nodes:
    - 10.0.0.55
    - 10.0.0.56
  orchestrate_nodes:
    auth:
      username: root
      password: toor
    nodes:
      - 10.0.0.1 # 👈 Can be the current one or completly remote
      - 10.0.0.2
globals:
  url: https://my.new.app/v1
  header_type:
    name: x-app-type
    value: production
Flows:
  base_url: @globals->url
  before_all:
    sign_up:
      uri: /auth/register
      method: POST
      data:
        email: user$_uid@email.com
        password: $_nanoid(18)
  after_all:
    sign_out:
      uri: /auth/logout
  sign_and_delete_post:
    base_url: https://my.new.app/v1 # 👈 can be set here as well
    sign_in:
      uri: /auth/login
      method: POST
      data:
        email: @before_all->sign_up->email
        password: @before_all->sign_up->password
    delete_post:
      url: https://fourm.new.app/post
      method: DELETE
      auth_from: @sign_and_delete_post->sign_in
      headers:
        - @globals->header_type
        - "x-header: value"
```

### available yaml variables

| var              | what                               | example             |
|------------------|------------------------------------|---------------------|
| $date            | `mm-dd-YYYY-HH-MM-SS`              | 10-24-2024-18-25-24 |
| $_uid            | increment ID                       | 1,2,3,4             |
| $_nanoid(length) | random nano id with desired length | afa3fq3Fg526        |
