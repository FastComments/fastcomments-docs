### Erreurs 401 Unauthorized

Si vous obtenez des erreurs 401 lors de l'utilisation de l'API authentifiée :

1. **Vérifiez votre clé API** : Assurez-vous d'utiliser la bonne clé API depuis votre tableau de bord FastComments
2. **Vérifiez l'ID du tenant** : Assurez-vous que l'ID du tenant correspond à votre compte
3. **Format de la clé API** : La clé API doit être transmise dans la Configuration :

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### Problèmes de jeton SSO

Si les jetons SSO ne fonctionnent pas :

1. **Utilisez le mode sécurisé en production** : Utilisez toujours `FastCommentsSSO::new_secure()` avec votre clé API en production
2. **Côté serveur uniquement** : Générez les jetons SSO sur votre serveur, n'exposez jamais votre clé API aux clients
3. **Vérifiez les données utilisateur** : Assurez-vous que tous les champs requis (id, email, username) sont fournis

### Erreurs de runtime asynchrone

Le SDK utilise tokio pour les opérations asynchrones. Assurez-vous de :

1. Ajouter tokio à vos dépendances :
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Utiliser le runtime tokio :
```rust
#[tokio::main]
async fn main() {
    // Votre code asynchrone ici
}
```