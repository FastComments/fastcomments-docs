---
### Erros 401 — Não autorizado

Se você está recebendo erros 401 ao usar a API autenticada:

1. **Verifique sua chave de API**: Certifique-se de que está usando a chave de API correta do painel do FastComments
2. **Verifique o tenant ID**: Garanta que o tenant ID corresponda à sua conta
3. **Formato da chave de API**: A chave de API deve ser passada na Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problemas com Tokens SSO

Se os tokens SSO não estiverem funcionando:

1. **Use o modo seguro em produção**: Sempre utilize `FastCommentsSSO::new_secure()` com sua chave de API em produção
2. **Somente no servidor**: Gere os tokens SSO no seu servidor, nunca exponha sua chave de API para clientes
3. **Verifique os dados do usuário**: Garanta que todos os campos obrigatórios (id, email, username) estejam fornecidos

### Erros do runtime assíncrono

O SDK usa tokio para operações assíncronas. Certifique-se de:

1. Adicione tokio às suas dependências:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use o runtime tokio:
```rust
#[tokio::main]
async fn main() {
    // Seu código assíncrono aqui
}
```
---