FastComments pokreće hostovani Model Context Protocol (MCP) server kako AI asistenti i agentski klijenti mogu direktno pozivati FastComments API. Svaki alat koji MCP server izlaže automatski je generisan iz javne OpenAPI specifikacije, tako da sve što REST API može, MCP klijent može.

Endpoint je stateless i zasnovan na streamable-HTTP. Ne postoji sesija koja se održava i nema server‑side stanja po klijentu.

### Endpoint

[inline-code-attrs-start title = 'MCP krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Bilo koji MCP klijent koji podržava udaljene servere putem OAuth-a (Claude, ChatGPT, Claude Code, Cursor i drugi) može se povezati na gornji endpoint bez ikakvog podešavanja na strani FastComments. Klijent se registruje putem Dynamic Client Registration ili se identifikuje pomoću Client ID Metadata Document, otvara pregledač kako biste se prijavili u FastComments i odobrili pristup, i prima token vezan za nalog na koji ste prijavljeni.

Dokumenti za otkrivanje se nalaze na standardnim lokacijama:

[inline-code-attrs-start title = 'Otkrivanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Vaš korisnik treba da ima API Admin dozvolu na nalogu da bi odobrio vezu. Ako upravljate sa više naloga, pre odobravanja prebacite se na pravi nalog u kontrolnoj tabli.

Klijent može zatražiti `read` opseg, `write` opseg, ili oba. Klijent koji ne zatraži ništa dobija oba. Alati koji menjaju podatke nisu dostupni tokenu samo za čitanje.

Kontrolna tabla ima pomoćnika za podešavanje sa spremnim isječcima za kopiranje. Otvorite **Integrate -> MCP Server**, ili posetite direktno:

[inline-code-attrs-start title = 'Stranica za podešavanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registrujte FastComments server jednim komandama, zatim pokrenite `/mcp` unutar sesije da se prijavite i prikažete dostupne alate:

[inline-code-attrs-start title = 'Claude Code podešavanje'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Dodajte ovaj blok u konfiguraciju MCP servera vašeg klijenta (`mcp.json` za Cursor). Klijent otvara pregledač da se prijavi pri prvom korišćenju.

[inline-code-attrs-start title = 'MCP konfiguracija klijenta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Svaka odobrena veza je navedena pod **Integrate -> Connected Apps** u kontrolnoj tabli. Opozivanjem jedne poništava se svaki token koji ta aplikacija drži. Aplikacije se registruju kada se povežu i FastComments ih ne pregledava, zato opozovite sve što ne prepoznajete.

### Using the token with the REST API

Pristupni token koji MCP klijent dobije je regularna FastComments API akreditacija. Radi na svakom `/api/v1` endpointu kao bearer token, tako da aplikacija koja se poveže putem MCP-a može direktno pozivati REST API:

[inline-code-attrs-start title = 'Bearer token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Tenant je impliciran tokenom. `tenantId` se i dalje može proslediti, ali mora da se podudara. `GET` zahtevi zahtevaju `read` opseg, a sve ostalo `write` opseg.

### Connect with an API key

Klijenti koji ne mogu da završe prijavu putem pregledača, kao što su headless serveri, mogu se autentifikovati pomoću API ključa. Prosledite `tenantId` i `API_KEY` kao parametre upita, ili kao `x-tenant-id` i `x-api-key` HTTP zaglavlja ako vaš klijent podržava prilagođena zaglavlja:

[inline-code-attrs-start title = 'API ključ endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Stranica za podešavanje generiše ovaj URL za svaki vaš API ključ.

### Security

URL endpointa koji sadrži API ključ je tajna: ne zalepite ga u javne četove, screenshotove ili commit‑ove. Ako je ključ izložen, rotirajte ga na stranici API ključeva u vašoj kontrolnoj tabli. OAuth tokeni ne nose takav rizik jer su vezani za jednu aplikaciju i mogu se opozvati iz Connected Apps.