### L'API FastComments

FastComments fournit une API pour interagir avec de nombreuses ressources. Créez des intégrations avec notre plateforme, ou même développez vos propres clients !

Dans cette documentation, vous trouverez toutes les ressources prises en charge par l'API documentées avec leurs types de requête et de réponse.

Pour les clients Enterprise, tout accès à l'API est enregistré dans le journal d'audit.

### SDKs générés

FastComments génère désormais une [Spécification d'API](https://fastcomments.com/js/swagger.json) à partir de notre code (ce n'est pas encore complet, mais inclut de nombreuses API).

Nous disposons désormais de SDKs pour les langages populaires :

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Authentification

L'API est authentifiée en transmettant votre [clé API](https://fastcomments.com/auth/my-account/api-secret) soit via un en-tête `X-API-KEY`, soit via un paramètre de requête `API_KEY`. Vous aurez également besoin de votre `tenantId` pour effectuer des appels à l'API. Celui-ci peut être récupéré depuis la même page que votre clé API.

### Note de sécurité

Ces routes sont destinées à être appelées depuis un **serveur**. __NE LES APPELEZ PAS__ depuis un navigateur. Le faire exposera votre clé API — cela donnera un accès complet à votre compte à toute personne pouvant voir le code source d'une page !

#### Option d'authentification 1 - En-têtes

- En-tête : `X-API-KEY`
- En-tête : `X-TENANT-ID`

#### Option d'authentification 2 - Paramètres de requête

- Paramètre de requête : `API_KEY`
- Paramètre de requête : `tenantId`