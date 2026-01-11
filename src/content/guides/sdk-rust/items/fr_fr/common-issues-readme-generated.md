### 401 Unauthorized Errors

Si vous obtenez des erreurs 401 lorsque vous utilisez l'API authentifiée :

1. **Vérifiez votre clé API** : Assurez-vous d'utiliser la clé API correcte depuis votre tableau de bord FastComments
2. **Vérifiez le tenant ID** : Assurez-vous que le tenant ID correspond à votre compte
3. **Format de la clé API** : La clé API doit être passée dans la Configuration :

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO Token Issues

Si les jetons SSO ne fonctionnent pas :

1. **Utilisez le mode sécurisé en production** : Toujours utiliser `FastCommentsSSO::new_secure()` avec votre clé API pour la production
2. **Côté serveur uniquement** : Générez les jetons SSO sur votre serveur, n'exposez jamais votre clé API aux clients
3. **Vérifiez les données utilisateur** : Assurez-vous que tous les champs requis (id, email, username) sont fournis

### Async Runtime Errors

Le SDK utilise tokio pour les opérations asynchrones. Assurez-vous de :

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