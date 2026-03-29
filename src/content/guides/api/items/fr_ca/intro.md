### L'API FastComments

FastComments fournit une API pour interagir avec de nombreuses ressources. Créez des intégrations avec notre plateforme, ou développez même vos propres clients !

Dans cette documentation, vous trouverez toutes les ressources prises en charge par l'API documentées avec leurs types de requête et de réponse.

Pour les clients Enterprise, tout accès à l'API est enregistré dans le journal d'audit.

### SDKs générés

FastComments génère désormais une [API Spec](https://fastcomments.com/js/swagger.json) à partir de notre code (ce n'est pas encore complet, mais inclut de nombreuses API).

Nous avons également maintenant des SDK pour des langages populaires :

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

L'API s'authentifie en passant votre [api key](https://fastcomments.com/auth/my-account/api-secret) soit dans l'en-tête `X-API-KEY`, soit dans le paramètre de requête `API_KEY`. Vous aurez également besoin de votre `tenantId` pour effectuer des appels API. Celui-ci peut être récupéré depuis la même page que votre clé API.

### Note de sécurité

Ces routes sont destinées à être appelées depuis un **serveur**. __NE PAS__ les appeler depuis un navigateur. Le faire exposera votre clé API — cela donnera un accès complet à votre compte à quiconque peut voir le code source d'une page !

#### Option d'authentification 1 - En-têtes

- En-tête : `X-API-KEY`
- En-tête : `X-TENANT-ID`

#### Option d'authentification 2 - Paramètres de requête

- Paramètre de requête : `API_KEY`
- Paramètre de requête : `tenantId`

### Lire vos propres écritures

FastComments offre une disponibilité active-active. Les requêtes depuis votre centre de données sont routées vers [le point de présence le plus proche](https://sophon.fastcomments.com/) du vôtre. Ceci est automatique, et normalement vous pouvez observer une sémantique lecture-après-écriture. Si vous voulez vous assurer de lire vos propres écritures, vous pouvez épingler vos requêtes à une certaine région en utilisant cette région comme hôte API (ceci n'est toutefois généralement pas nécessaire pour la plupart des intégrations) :

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Notez que si vous faites cela, vous voudrez peut-être définir un mécanisme de repli, car nous avons mis hors service des nœuds d'entrée par le passé et utilisé de nouveaux noms lors du basculement.