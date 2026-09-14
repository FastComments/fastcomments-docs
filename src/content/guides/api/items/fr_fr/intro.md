### L'API FastComments

FastComments fournit une API pour interagir avec de nombreuses ressources. Créez des intégrations avec notre plateforme, ou même développez vos propres clients !

Dans cette documentation, vous trouverez toutes les ressources prises en charge par l'API, documentées avec leurs types de requêtes et de réponses.

Pour les clients Enterprise, tout accès à l'API est enregistré dans le journal d'audit.

### SDK générés

FastComments génère désormais une [spécification d'API](https://fastcomments.com/js/swagger.json) à partir de notre code (ceci n'est pas encore complet, mais inclut de nombreuses API).

Nous disposons également maintenant de SDK pour les langages populaires :

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

L'API est authentifiée en transmettant votre [clé API](https://fastcomments.com/auth/my-account/api-secret) soit dans l'en-tête `X-API-KEY`, soit comme paramètre de requête `API_KEY`. Vous aurez également besoin de votre `tenantId` pour effectuer les appels API. Celui-ci peut être récupéré sur la même page que votre clé API.

### Note de sécurité

Ces routes sont destinées à être appelées depuis un **serveur**. __NE LES APPELEZ PAS__ depuis un navigateur. Le faire exposera votre clé API — cela donnera un accès complet à votre compte à quiconque peut voir le code source d'une page !

#### Option d'authentification 1 – En‑têtes

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Option d'authentification 2 – Paramètres de requête

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Option d'authentification 3 – Jeton Bearer OAuth

- Header: `Authorization: Bearer fcat_...`

Les applications tierces telles que Zapier et les clients du [serveur MCP](https://docs.fastcomments.com/guide-llm-kit.html) obtiennent un jeton via OAuth au lieu d'une clé API. Ce jeton fonctionne sur chaque point de terminaison ici. Le locataire est implicite dans le jeton, donc `tenantId` est optionnel, mais il doit correspondre au jeton lorsqu'il est fourni. Les requêtes `GET` nécessitent le scope `read` et toutes les autres méthodes nécessitent le scope `write`. Le flux complet, incluant l'enregistrement du client, PKCE, rafraîchissement et révocation, est documenté sous [OAuth Authorization](#oauth). La découverte commence à `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Lecture de vos propres écritures

FastComments offre une disponibilité Active‑Active. Les requêtes depuis votre centre de données sont routées vers le [point de présence le plus proche](https://sophon.fastcomments.com/) du vôtre. Cela se fait automatiquement, et normalement vous pouvez observer la sémantique « lecture‑après‑écriture ». Si vous souhaitez vous assurer de lire vos propres écritures, vous pouvez ancrer vos requêtes à une région spécifique en utilisant cette région comme hôte API (cependant cela n'est généralement pas nécessaire pour la plupart des intégrations) :

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Notez que si vous faites cela, vous voudrez peut‑être définir une solution de secours, car nous avons déprécié des nœuds d'entrée par le passé et utilisons de nouveaux noms pour le basculement.