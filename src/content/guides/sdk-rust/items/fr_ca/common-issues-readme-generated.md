### 401 Unauthorized Errors

If you're getting 401 errors when using the authenticated API:

1. **Vérifiez votre clé API** : Assurez-vous d'utiliser la bonne clé API depuis votre FastComments dashboard
2. **Vérifiez l'ID du locataire** : Assurez-vous que l'ID du locataire correspond à votre compte
3. **Format de la clé API** : La clé API doit être fournie dans la Configuration :

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO Token Issues

If SSO tokens aren't working:

1. **Utilisez le mode sécurisé en production** : Utilisez toujours `FastCommentsSSO::new_secure()` avec votre clé API en production
2. **Côté serveur uniquement** : Générez les jetons SSO sur votre serveur, n'exposez jamais votre clé API aux clients
3. **Vérifiez les données utilisateur** : Assurez-vous que tous les champs requis (id, email, username) sont fournis

### Async Runtime Errors

The SDK uses tokio for async operations. Make sure to:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Votre code asynchrone ici
}
```