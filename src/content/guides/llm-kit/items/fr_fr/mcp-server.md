FastComments exécute un serveur Model Context Protocol (MCP) hébergé afin que les assistants IA et les clients agentiques puissent appeler directement l'API FastComments. Chaque outil exposé par le serveur MCP est généré automatiquement à partir de la spécification OpenAPI publique, de sorte que tout ce que l'API REST peut faire, un client MCP peut le faire.

L'endpoint est sans état et basé sur HTTP streamable. Il n'y a aucune session à maintenir et aucun état côté serveur par client.

### Endpoint

[inline-code-attrs-start title = 'Point de terminaison MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Tout client MCP qui prend en charge les serveurs distants avec OAuth (Claude, ChatGPT, Claude Code, Cursor et d'autres) peut se connecter à l'endpoint ci‑dessus sans configuration côté FastComments. Le client s'enregistre via l'enregistrement dynamique de client ou s'identifie avec un document de métadonnées d'ID client, ouvre un navigateur afin que vous puissiez vous connecter à FastComments et approuver l'accès, et reçoit un jeton lié au compte avec lequel vous étiez connecté.

Les documents de découverte se trouvent aux emplacements standard :

[inline-code-attrs-start title = 'Découverte'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Votre utilisateur doit disposer de l'autorisation API Admin sur le compte pour approuver une connexion. Si vous gérez plusieurs comptes, basculez vers le bon dans le tableau de bord avant d'approuver.

Un client peut demander le scope `read`, le scope `write`, ou les deux. Un client qui ne demande rien obtient les deux. Les outils qui modifient les données ne sont pas proposés à un jeton en lecture seule.

Le tableau de bord possède un assistant de configuration avec des extraits prêts à coller. Ouvrez **Intégrer -> Serveur MCP**, ou accédez-y directement :

[inline-code-attrs-start title = 'Page de configuration'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Enregistrez le serveur FastComments avec une seule commande, puis exécutez `/mcp` dans une session pour vous connecter et lister les outils disponibles :

[inline-code-attrs-start title = 'Configuration Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Ajoutez ce bloc à la configuration des serveurs MCP de votre client (`mcp.json` pour Cursor). Le client ouvre un navigateur pour se connecter lors de la première utilisation.

[inline-code-attrs-start title = 'Configuration client MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Revoking access

Toute connexion approuvée est répertoriée sous **Intégrer -> Applications connectées** dans le tableau de bord. Révoquer une connexion invalide tous les jetons détenus par cette application. Les applications s'enregistrent lorsqu'elles se connectent et FastComments ne les examine pas, donc révoquez tout ce que vous ne reconnaissez pas.

### Using the token with the REST API

Le jeton d'accès qu'un client MCP obtient est une credential API FastComments standard. Il fonctionne sur chaque endpoint `/api/v1` en tant que token Bearer, ainsi une application connectée via MCP peut également appeler directement l'API REST :

[inline-code-attrs-start title = 'Token Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Le locataire est implicite dans le token. Un `tenantId` peut encore être fourni mais il doit correspondre. Les requêtes `GET` nécessitent le scope `read` et toutes les autres nécessitent le scope `write`.

### Connect with an API key

Les clients qui ne peuvent pas effectuer une connexion via navigateur, comme les serveurs sans tête, peuvent s'authentifier avec une clé API à la place. Transmettez `tenantId` et `API_KEY` en tant que paramètres de requête, ou comme en‑têtes HTTP `x-tenant-id` et `x-api-key` si votre client prend en charge les en‑têtes personnalisés :

[inline-code-attrs-start title = 'Endpoint clé API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

La page de configuration génère cette URL pour chacune de vos clés API.

### Security

Une URL d'endpoint contenant une clé API est un secret : ne la collez pas dans des discussions publiques, captures d'écran ou commits. Si une clé est exposée, renouvelez‑la sur la page Clés API de votre tableau de bord. Les jetons OAuth ne présentent pas ce risque car ils sont liés à une seule application et peuvent être révoqués depuis Applications connectées.