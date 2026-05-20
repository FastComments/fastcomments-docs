FastComments exécute un serveur hébergé Model Context Protocol (MCP) afin que les assistants de codage IA et les clients agentiques puissent appeler l'API FastComments directement. Tous les outils que le MCP server expose sont générés automatiquement à partir du OpenAPI spec public, donc tout ce que l'API REST peut faire, un client MCP peut le faire.

The endpoint est sans état et basé sur HTTP streamable. Il n'y a pas de session à maintenir, pas d'étape d'enregistrement du client, et pas d'état côté serveur par client.

### Endpoint

[inline-code-attrs-start title = 'Point de terminaison MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

L'authentification utilise la même API key que l'API REST. Vous pouvez aussi passer `tenantId` et la clé en tant qu'en-têtes HTTP `x-tenant-id` et `x-api-key` si votre client prend en charge les en-têtes personnalisés.

### Pre-filled setup

Le tableau de bord propose un assistant de configuration qui génère l'URL et des extraits de configuration prêts à coller pour les clients MCP populaires. Allez dans le tableau de bord de votre compte et ouvrez **Integrate -> MCP Server**, ou visitez-le directement :

[inline-code-attrs-start title = 'Page de configuration'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Choisissez la clé API à utiliser dans le menu déroulant, puis copiez l'un des extraits générés.

### Claude Code

Enregistrez le serveur FastComments avec une commande :

[inline-code-attrs-start title = 'Configuration Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Après son enregistrement, exécutez `/mcp` dans une session Claude Code pour confirmer la connexion et lister les outils disponibles.

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

La clé API est intégrée dans l'URL. Traitez l'URL comme un secret : ne la collez pas dans des chats publics, des captures d'écran ou des commits. Si une clé est exposée, renouvelez-la sur la page Clés API de votre tableau de bord.