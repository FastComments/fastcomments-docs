FastComments exploite un serveur hébergé Model Context Protocol (MCP) afin que les assistants de codage IA et les clients agentiques puissent appeler l'API FastComments directement. Chaque outil exposé par le serveur MCP est généré automatiquement à partir de la spécification OpenAPI publique, donc tout ce que l'API REST peut faire, un client MCP peut le faire.

Le point de terminaison est sans état et basé sur HTTP en streaming. Il n'y a pas de session à maintenir, pas d'étape d'enregistrement du client, et aucun état côté serveur par client.

### Endpoint

[inline-code-attrs-start title = 'Point de terminaison MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

L'authentification utilise la même clé API que l'API REST. Vous pouvez également transmettre `tenantId` et la clé via les en-têtes HTTP `x-tenant-id` et `x-api-key` si votre client prend en charge les en-têtes personnalisés.

### Pre-filled setup

Le tableau de bord propose un assistant de configuration qui génère l'URL et des extraits de configuration prêts à coller pour les clients MCP populaires. Accédez au tableau de bord de votre compte et ouvrez **Intégrer -> Serveur MCP**, ou visitez-le directement :

[inline-code-attrs-start title = 'Page de configuration'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Choisissez la clé API à utiliser dans le menu déroulant, puis copiez l'un des extraits générés.

### Claude Code

Enregistrez le serveur FastComments avec une seule commande :

[inline-code-attrs-start title = 'Configuration de Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Une fois enregistré, exécutez `/mcp` dans une session Claude Code pour confirmer la connexion et lister les outils disponibles.

### Claude Desktop / Cursor

Ajoutez ce bloc à la configuration des serveurs MCP de votre client (`claude_desktop_config.json` pour Claude Desktop, `mcp.json` pour Cursor) :

[inline-code-attrs-start title = 'Configuration du client MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### Security

La clé API est intégrée dans l'URL. Traitez l'URL comme un secret : ne la collez pas dans des discussions publiques, des captures d'écran ou des commits. Si une clé est exposée, faites-la pivoter sur la page des clés API de votre tableau de bord.