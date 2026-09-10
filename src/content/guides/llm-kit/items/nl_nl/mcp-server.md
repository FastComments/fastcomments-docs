FastComments draait een gehoste Model Context Protocol (MCP) server zodat AI‑assistenten en agentische clients de FastComments API direct kunnen aanroepen. Elke tool die de MCP‑server beschikbaar stelt, wordt automatisch gegenereerd vanuit de openbare OpenAPI‑specificatie, dus alles wat de REST API kan doen, kan een MCP‑client doen.

Het eindpunt is stateless en gebaseerd op streamable‑HTTP. Er is geen sessie die actief moet blijven en geen server‑side status per client.

### Endpoint

[inline-code-attrs-start title = 'MCP-eindpunt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Verbinden met OAuth

Elke MCP‑client die externe servers met OAuth ondersteunt (Claude, ChatGPT, Claude Code, Cursor en anderen) kan verbinding maken met het bovenstaande eindpunt zonder enige configuratie aan de FastComments‑kant. De client registreert zichzelf via Dynamic Client Registration of identificeert zichzelf met een Client ID Metadata Document, opent een browser zodat je kunt inloggen bij FastComments en toegang kunt goedkeuren, en ontvangt een token dat gekoppeld is aan het account waarmee je bent ingelogd.

De discovery‑documenten bevinden zich op de standaardlocaties:

[inline-code-attrs-start title = 'Ontdekking'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Je gebruiker heeft de API Admin‑rechten op het account nodig om een verbinding goed te keuren. Als je meerdere accounts beheert, schakel dan in het dashboard naar het juiste account voordat je goedkeurt.

Een client kan de `read` scope, de `write` scope, of beide aanvragen. Een client die niets aanvraagt, krijgt beide. Tools die gegevens wijzigen, worden niet aangeboden aan een token met alleen‑lezen rechten.

Het dashboard heeft een installatiehulp met kant‑en‑klare snippets om te plakken. Open **Integrate -> MCP Server**, of bezoek het direct:

[inline-code-attrs-start title = 'Instellingspagina'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registreer de FastComments‑server met één commando, en voer vervolgens `/mcp` uit binnen een sessie om in te loggen en de beschikbare tools te tonen:

[inline-code-attrs-start title = 'Claude Code Instelling'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor en andere config‑bestand clients

Voeg dit blok toe aan de MCP‑serversconfiguratie van je client (`mcp.json` voor Cursor). De client opent een browser om bij het eerste gebruik in te loggen.

[inline-code-attrs-start title = 'MCP-clientconfiguratie'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Toegang intrekken

Elke goedgekeurde verbinding wordt weergegeven onder **Integrate -> Connected Apps** in het dashboard. Het intrekken van één verbinding maakt elk token dat die applicatie bezit ongeldig. Applicaties registreren zichzelf wanneer ze verbinding maken en FastComments beoordeelt ze niet, dus trek alles in dat je niet herkent.

### Het token gebruiken met de REST API

Het toegangstoken dat een MCP‑client verkrijgt, is een reguliere FastComments API‑credential. Het werkt op elk `/api/v1` eindpunt als een bearer‑token, zodat een applicatie die via MCP is verbonden ook direct de REST API kan aanroepen:

[inline-code-attrs-start title = 'Bearer-token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

De tenant wordt afgeleid uit het token. Een `tenantId` kan nog steeds worden meegegeven, maar moet overeenkomen. `GET`‑verzoeken hebben de `read` scope nodig en alle andere verzoeken hebben de `write` scope nodig.

### Verbinden met een API‑sleutel

Clients die geen browseraanmelding kunnen voltooien, zoals headless servers, kunnen in plaats daarvan authenticeren met een API‑sleutel. Geef `tenantId` en `API_KEY` mee als query‑parameters, of als de `x-tenant-id` en `x-api-key` HTTP‑headers als je client aangepaste headers ondersteunt:

[inline-code-attrs-start title = 'API-sleutel eindpunt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

De instellingspagina genereert deze URL voor elk van je API‑sleutels.

### Beveiliging

Een eindpunt‑URL die een API‑sleutel bevat, is een geheim: plak deze niet in openbare chats, screenshots of commits. Als een sleutel wordt blootgesteld, roteer deze op de API‑sleutels‑pagina in je dashboard. OAuth‑tokens dragen dit risico niet omdat ze gebonden zijn aan één applicatie en kunnen worden ingetrokken via Connected Apps.