FastComments kører en hostet Model Context Protocol (MCP) server, så AI-kodeassistenter og agentiske klienter kan kalde FastComments API'et direkte. Alle værktøjer, som MCP-serveren eksponerer, er autogenereret fra den offentlige OpenAPI-specifikation, så alt hvad REST-API'et kan gøre, kan en MCP-klient også gøre.

Endpointet er uden tilstand og baseret på streaming-HTTP. Der er ingen session at holde i live, ingen klientregistreringstrin, og ingen serverside-tilstand per klient.

### Endpoint

[inline-code-attrs-start title = 'MCP-endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Autentificering bruger den samme API-nøgle som REST-API'et. Du kan også sende `tenantId` og nøglen som `x-tenant-id` og `x-api-key` HTTP-headere, hvis din klient understøtter brugerdefinerede headere.

### Pre-filled setup

Dashboardet har en opsætningshjælper, der genererer URL'en og klar-til-indsætning konfigurationsuddrag for populære MCP-klienter. Gå til dit konto-dashboard og åbn **Integrate -> MCP Server**, eller besøg det direkte:

[inline-code-attrs-start title = 'Opsætningsside'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Vælg hvilken API-nøgle der skal bruges fra dropdown-menuen, og kopier derefter et af de genererede uddrag.

### Claude Code

Registrer FastComments-serveren med én kommando:

[inline-code-attrs-start title = 'Opsætning af Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Efter den er registreret, kør `/mcp` i en Claude Code-session for at bekræfte forbindelsen og liste tilgængelige værktøjer.

### Claude Desktop / Cursor

Tilføj denne blok til din klients MCP-serverkonfiguration (`claude_desktop_config.json` for Claude Desktop, `mcp.json` for Cursor):

[inline-code-attrs-start title = 'MCP-klientkonfiguration'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

API-nøglen er indlejret i URL'en. Behandl URL'en som en hemmelighed: indsæt den ikke i offentlige chats, screenshots eller commits. Hvis en nøgle bliver eksponeret, roter den på siden API Keys i dit dashboard.