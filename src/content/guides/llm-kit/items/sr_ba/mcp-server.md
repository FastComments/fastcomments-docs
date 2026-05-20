FastComments pokreće hostovani Model Context Protocol (MCP) server tako da AI asistenti za kodiranje i agentni klijenti mogu direktno pozivati FastComments API. Svaki alat koji MCP server izlaže je automatski generisan iz javne OpenAPI specifikacije, tako da sve što REST API može da uradi, MCP klijent takođe može da uradi.

Endpoint je bez stanja i zasnovan na streamable-HTTP-u. Nema sesije koju treba održavati, nema koraka registracije klijenta, i nema stanja na serverskoj strani po klijentu.

### Endpoint

[inline-code-attrs-start title = 'MCP Krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Autentifikacija koristi isti API ključ kao REST API. Takođe možete poslati `tenantId` i ključ kao HTTP zaglavlja `x-tenant-id` i `x-api-key` ako vaš klijent podržava prilagođena zaglavlja.

### Pre-filled setup

Kontrolna tabla ima pomoćnika za postavljanje koji generiše URL i isječke konfiguracije spremne za lijepljenje za popularne MCP klijente. Idite na kontrolnu tablu vašeg naloga i otvorite **Integrate -> MCP Server**, ili posjetite direktno:

[inline-code-attrs-start title = 'Stranica za postavljanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Odaberite koji API ključ ćete koristiti iz padajućeg menija, zatim kopirajte bilo koji od generisanih isječaka.

### Claude Code

Registrujte FastComments server jednom komandom:

[inline-code-attrs-start title = 'Podešavanje Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Nakon što je registrovan, pokrenite `/mcp` unutar Claude Code sesije da potvrdite vezu i prikažete dostupne alate.

### Claude Desktop / Cursor

Dodajte ovaj blok u konfiguraciju MCP servera vašeg klijenta (`claude_desktop_config.json` za Claude Desktop, `mcp.json` za Cursor):

[inline-code-attrs-start title = 'MCP Konfiguracija klijenta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

API ključ je ugrađen u URL. Postupajte sa URL-om kao sa tajnom: ne lijepite ga u javne razgovore, snimke ekrana ili commitove. Ako je ključ otkriven, rotirajte ga na stranici API ključeva u vašoj kontrolnoj tabli.