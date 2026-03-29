### L'API FastComments

FastComments fournit une API pour interagir avec de nombreuses ressources. Créez des intégrations avec notre plateforme, ou même développez vos propres clients !

Dans cette documentation, vous trouverez toutes les ressources prises en charge par l'API, documentées avec leurs types de requête et de réponse.

Pour les clients Enterprise, tout accès à l'API est consigné dans le journal d'audit.

### SDKs générés

FastComments génère maintenant une [spécification d'API](https://fastcomments.com/js/swagger.json) à partir de notre code (ceci n'est pas encore complet, mais inclut de nombreuses API).

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

L'API s'authentifie en passant votre [clé d'API](https://fastcomments.com/auth/my-account/api-secret) soit via l'en-tête `X-API-KEY`, soit via le paramètre de requête `API_KEY`. Vous aurez également besoin de votre `tenantId` pour effectuer des appels API. Celui-ci peut être récupéré depuis la même page que votre clé d'API.

### Remarque de sécurité

Ces routes doivent être appelées depuis un **serveur**. __NE PAS__ les appeler depuis un navigateur. Le faire exposera votre clé d'API — cela donnera un accès complet à votre compte à toute personne pouvant voir le code source d'une page !

#### Option d'authentification un - En-têtes

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Option d'authentification deux - Paramètres de requête

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Lecture de vos propres écritures

FastComments fournit une disponibilité active-active. Les requêtes depuis votre centre de données sont routées vers [le point de présence le plus proche](https://sophon.fastcomments.com/) du vôtre. Ceci est automatique, et normalement vous pouvez observer la sémantique lecture-après-écriture. Si vous souhaitez vous assurer de lire vos propres écritures, vous pouvez fixer vos requêtes sur une région donnée en utilisant cette région comme hôte API (cependant cela n'est généralement pas nécessaire pour la plupart des intégrations) :

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Notez que si vous faites cela, vous voudrez peut-être définir un mécanisme de secours, car nous avons déprécié des nœuds d'entrée par le passé et utilisé de nouveaux noms lors des basculements.