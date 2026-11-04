FastComments héberge un serveur Model Context Protocol (MCP) afin que les assistants IA et les clients agentiques puissent appeler l'API FastComments directement. Chaque outil exposé par le serveur MCP est généré automatiquement à partir de la spécification OpenAPI publique, donc tout ce que l'API REST peut faire, un client MCP peut le faire.

Le endpoint est sans état et basé sur streamable-HTTP. Il n'y a aucune session à maintenir et aucun état côté serveur par client.

### Endpoint

[inline-code-attrs-start title = 'Endpoint MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connexion avec OAuth

Tout client MCP prenant en charge les serveurs distants avec OAuth (Claude, ChatGPT, Claude Code, Cursor, et autres) peut se connecter au endpoint ci-dessus sans aucune configuration côté FastComments. Le client s'enregistre via Dynamic Client Registration ou s'identifie avec un Client ID Metadata Document, ouvre un navigateur pour que vous puissiez vous connecter à FastComments et approuver l'accès, puis reçoit un token lié au compte auquel vous étiez connecté.

Les documents de découverte se trouvent aux emplacements standards :

[inline-code-attrs-start title = 'Découverte'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Votre utilisateur doit disposer de la permission API Admin sur le compte pour approuver une connexion. Si vous gérez plusieurs comptes, basculez vers le bon compte dans le dashboard avant d'approuver.

Un client peut demander le scope `read`, le scope `write`, ou les deux. Un client qui ne demande rien obtient les deux. Les outils qui modifient des données ne sont pas proposés à un token en lecture seule.

Le dashboard dispose d'un assistant de configuration avec des snippets prêts à copier. Ouvrez **Integrate -> MCP Server**, ou visitez directement :

[inline-code-attrs-start title = 'Page de configuration'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Enregistrez le serveur FastComments avec une seule commande, puis exécutez `/mcp` dans une session pour vous connecter et lister les outils disponibles :

[inline-code-attrs-start title = 'Configuration de Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor et autres clients à fichier de configuration

Ajoutez ce bloc à la configuration des serveurs MCP de votre client (`mcp.json` pour Cursor). Le client ouvre un navigateur pour se connecter lors de la première utilisation.

[inline-code-attrs-start title = 'Configuration du client MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### Révocation de l'accès

Chaque connexion approuvée est listée sous **Integrate -> Connected Apps** dans le dashboard. Révoquer une connexion invalide tous les tokens détenus par cette application. Les applications s'enregistrent elles-mêmes lorsqu'elles se connectent et FastComments ne les examine pas, donc révoquez tout ce que vous ne reconnaissez pas.

### Utilisation du token avec l'API REST

Le token d'accès qu'un client MCP obtient est un identifiant FastComments API ordinaire. Il fonctionne sur chaque endpoint `/api/v1` en tant que bearer token, donc une application connectée via MCP peut aussi appeler l'API REST directement :

[inline-code-attrs-start title = 'Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Le tenant est déduit du token. Un `tenantId` peut toujours être passé mais il doit correspondre. Les requêtes `GET` nécessitent le scope `read` et tout le reste nécessite le scope `write`.

### Connexion avec une clé API

Les clients qui ne peuvent pas effectuer de connexion via navigateur, comme les serveurs headless, peuvent s'authentifier avec une clé API à la place. Passez `tenantId` et `API_KEY` en paramètres de requête, ou en tant qu'en-têtes HTTP `x-tenant-id` et `x-api-key` si votre client prend en charge les en-têtes personnalisés :

[inline-code-attrs-start title = 'Endpoint avec clé API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

La page de configuration génère cette URL pour chacune de vos clés API.

### Sécurité

Une URL de endpoint contenant une clé API est un secret : ne la collez pas dans des chats publics, des captures d'écran ou des commits. Si une clé est exposée, faites-la pivoter sur la page API Keys de votre dashboard. Les tokens OAuth ne présentent pas ce risque car ils sont liés à une seule application et peuvent être révoqués depuis Connected Apps.