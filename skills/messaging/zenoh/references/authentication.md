# Autenticação no Zenoh

A autenticação no Zenoh é configurada via `Config`, não através de tipos Rust específicos. Veja os métodos disponíveis:

## Configuração via JSON

### User-Password Authentication

```json
{
  "connect": {
    "endpoints": ["tcp/127.0.0.1:7447"],
    "auth": {
      "user_password": {
        "user": "admin",
        "password": "password123"
      }
    }
  }
}
```

### TLS Authentication

```json
{
  "connect": {
    "endpoints": ["tls/127.0.0.1:7447"],
    "tls": {
      "client_cert": "/path/to/cert.pem",
      "client_key": "/path/to/key.pem",
      "server_cert": "/path/to/server_cert.pem"
    }
  }
}
```

### API Config

```rust
use zenoh::config::Config;

let mut config = Config::default();
config.connect.endpoints.push("tcp/127.0.0.1:7447".parse()?);

// User-Password
config.auth.user_password.insert("user".to_string(), "password".to_string());
```

## Métodos de Autenticação Suportados

- **user_password**: Autenticação básica usuário/senha
- **tls**: Certificados TLS mútuos
- **none**: Sem autenticação (padrão)

## Access Control (ACL)

O controle de acesso é configurado via arquivo JSON separado:

```json
{
  "plugins": {
    "access_control": {
      "enabled": true,
      "default_permission": "deny",
      "rules": [
        {
          "keyexpr": "demo/**",
          "permission": "allow",
          "principals": ["user1"]
        }
      ]
    }
  }
}
```

## Veja Também

- [Zenoh User-Password Auth](https://zenoh.io/docs/manual/user-password/)
- [Zenoh TLS Auth](https://zenoh.io/docs/manual/tls/)
- [Zenoh Access Control](https://zenoh.io/docs/manual/access-control/)
