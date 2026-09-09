### L'API FastComments

FastComments fournit une API pour interagir avec de nombreuses ressources. Créez des intégrations avec notre plateforme, ou même développez vos propres clients !

Dans cette documentation, vous trouverez toutes les ressources prises en charge par l'API, documentées avec leurs types de requêtes et de réponses.

Pour les clients Enterprise, tout accès à l'API est consigné dans le journal d’audit.

### SDK générés

FastComments génère désormais une [API Spec](https://fastcomments.com/js/swagger.json) à partir de notre code (ceci n’est pas encore complet, mais inclut de nombreuses API).

Nous disposons également de SDK pour les langages populaires :

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

L'API est authentifiée en transmettant votre [clé d'API](https://fastcomments.com/auth/my-account/api-secret) soit dans un en‑tête `X-API-KEY`, soit dans le paramètre de requête `API_KEY`. Vous aurez également besoin de votre `tenantId` pour effectuer les appels API. Celui‑ci peut être récupéré sur la même page que votre clé d'API.

### Note de sécurité

Ces routes sont destinées à être appelées depuis un **serveur**. __NE LES APPELEZ PAS__ depuis un navigateur. Le faire exposerait votre clé d'API — cela donnerait un accès complet à votre compte à quiconque peut voir le code source d’une page !

#### Option d'authentification 1 - En‑têtes

- En‑tête : `X-API-KEY`
- En‑tête : `X-TENANT-ID`

#### Option d'authentification 2 - Paramètres de requête

- Paramètre de requête : `API_KEY`
- Paramètre de requête : `tenantId`

#### Option d'authentification 3 - Jeton d'accès OAuth

- En‑tête : `Authorization: Bearer fcat_...`

Les applications qui se connectent via le [serveur MCP](https://docs.fastcomments.com/guide-llm-kit.html) obtiennent un jeton via OAuth au lieu d’une clé d'API. Ce jeton fonctionne sur chaque point de terminaison ici. Le tenant est implicite dans le jeton, donc `tenantId` est optionnel, mais il doit correspondre au jeton lorsqu'il est fourni. Les requêtes `GET` nécessitent le scope `read` et toutes les autres méthodes nécessitent le scope `write`. La découverte commence à `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Lecture de vos propres écritures

FastComments offre une disponibilité Active‑Active. Les requêtes depuis votre centre de données sont routées vers le [point de présence le plus proche](https://sophon.fastcomments.com/) du vôtre. Cela se fait automatiquement, et normalement vous pouvez observer la sémantique « lecture‑écriture ». Si vous souhaitez vous assurer de lire vos propres écritures, vous pouvez épingler vos requêtes à une région donnée en utilisant cette région comme hôte API (bien que cela ne soit généralement pas nécessaire pour la plupart des intégrations) :

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Notez que si vous faites cela, vous pourriez vouloir définir une solution de secours, car nous avons déjà déprécié certains nœuds d’entrée et utilisons de nouveaux noms pour le basculement.