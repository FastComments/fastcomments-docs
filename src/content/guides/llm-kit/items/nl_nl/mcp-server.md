FastComments draait een gehoste Model Context Protocol (MCP)-server, zodat AI-codeassistenten en agentische clients de FastComments API rechtstreeks kunnen aanroepen. Elk hulpmiddel dat de MCP-server blootstelt, wordt automatisch gegenereerd vanuit de openbare OpenAPI-specificatie, dus alles wat de REST API kan doen, kan een MCP-client ook doen.

Het eindpunt is stateless en gebaseerd op streamable HTTP. Er is geen sessie om in stand te houden, geen stap voor client-registratie en geen server-side status per client.

### Eindpunt

[inline-code-attrs-start title = 'MCP-eindpunt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Authenticatie gebruikt dezelfde API-sleutel als de REST API. Je kunt ook `tenantId` en de sleutel doorgeven als de HTTP-headers `x-tenant-id` en `x-api-key` als je client aangepaste headers ondersteunt.

### Vooraf ingevulde configuratie

Het dashboard heeft een setup-hulpmiddel dat de URL en kant-en-klare configuratiesnippets voor populaire MCP-clients genereert. Ga naar je accountdashboard en open **Integrate -> MCP Server**, of bezoek deze direct:

[inline-code-attrs-start title = 'Setup-pagina'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Kies welke API-sleutel je wilt gebruiken uit het dropdownmenu, en kopieer daarna een van de gegenereerde snippets.

### Claude Code

Registreer de FastComments-server met één commando:

[inline-code-attrs-start title = 'Claude Code-installatie'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Nadat het geregistreerd is, voer `/mcp` uit in een Claude Code-sessie om de verbinding te bevestigen en beschikbare tools te tonen.

### Claude Desktop / Cursor

Voeg dit blok toe aan de MCP-serversconfiguratie van je client (`claude_desktop_config.json` voor Claude Desktop, `mcp.json` voor Cursor):

[inline-code-attrs-start title = 'MCP-clientconfiguratie'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Beveiliging

De API-sleutel is ingebed in de URL. Behandel de URL als een geheim: plak deze niet in openbare chats, screenshots of commits. Als een sleutel is blootgesteld, roteer deze op de API Keys-pagina in je dashboard.