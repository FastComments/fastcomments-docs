FastComments pokreće hostovani Model Context Protocol (MCP) server tako da AI pomoćnici za kodiranje i agentni klijenti mogu direktno pozivati FastComments API. Svaki alat koji MCP server izlaže automatski je generisan iz javne OpenAPI specifikacije, tako da sve što REST API može, i MCP klijent može.

Krajnja tačka je bezstanja i zasnovana na streamovanom HTTP-u. Ne postoji sesija koju treba održavati, nema koraka registracije klijenta i nema server-side stanja po klijentu.

### Endpoint

[inline-code-attrs-start title = 'MCP krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Autentifikacija koristi isti API ključ kao REST API. Takođe možete poslati `tenantId` i ključ kao HTTP zaglavlja `x-tenant-id` i `x-api-key` ako vaš klijent podržava prilagođena zaglavlja.

### Unapred pripremljeno podešavanje

U kontrolnoj tabli postoji pomoćnik za podešavanje koji generiše URL i gotove konfiguracione fragmente spremne za lepljenje za popularne MCP klijente. Idite na svoju kontrolnu tablu naloga i otvorite **Integrate -> MCP Server**, ili posetite direktno:

[inline-code-attrs-start title = 'Stranica za podešavanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Odaberite API ključ iz padajućeg menija, zatim kopirajte bilo koji od generisanih isječaka.

### Claude Code

Registrujte FastComments server jednom komandom:

[inline-code-attrs-start title = 'Podešavanje za Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Nakon registracije pokrenite `/mcp` unutar Claude Code sesije da potvrdite vezu i prikažete dostupne alate.

### Claude Desktop / Cursor

Dodajte ovaj blok u konfiguraciju MCP servera vašeg klijenta (`claude_desktop_config.json` za Claude Desktop, `mcp.json` za Cursor):

[inline-code-attrs-start title = 'Konfiguracija MCP klijenta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Bezbednost

API ključ je ugrađen u URL. Tretirajte URL kao tajnu: ne delite ga u javnim chatovima, snimcima ekrana ili commit-ovima. Ako je ključ izložen, rotirajte ga na stranici API Keys u vašoj kontrolnoj tabli.