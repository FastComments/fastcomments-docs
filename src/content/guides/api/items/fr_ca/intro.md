### L'API FastComments

FastComments fournit une API pour interagir avec de nombreuses ressources. Créez des intégrations avec notre plateforme, ou même créez vos propres clients!

Dans cette documentation, vous trouverez toutes les ressources prises en charge par l'API documentées avec leurs types de requête et de réponse.

Pour les clients Enterprise, tout l'accès à l'API est enregistré dans le journal d'audit.

### SDKs générés

FastComments génère maintenant une [Spécification d'API](https://fastcomments.com/js/swagger.json) à partir de notre code (ceci n'est pas encore complet, mais inclut de nombreuses API).

Nous disposons également maintenant de SDKs pour des langages populaires :

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

L'API s'authentifie en passant votre [clé API](https://fastcomments.com/auth/my-account/api-secret) soit comme en-tête `X-API-KEY`, soit comme paramètre de requête `API_KEY`. Vous aurez également besoin de votre `tenantId`
pour effectuer des appels API. Celui-ci peut être récupéré depuis la même page que votre clé API.

### Remarque de sécurité

Ces routes doivent être appelées depuis un **serveur**. __NE PAS__ les appeler depuis un navigateur. Le faire exposera votre clé API — cela donnera un accès complet à votre compte
à toute personne pouvant consulter le code source d'une page!

#### Option d'authentification 1 - En-têtes

- En-tête : `X-API-KEY`
- En-tête : `X-TENANT-ID`

#### Option d'authentification 2 - Paramètres de requête

- Paramètre de requête : `API_KEY`
- Paramètre de requête : `tenantId`