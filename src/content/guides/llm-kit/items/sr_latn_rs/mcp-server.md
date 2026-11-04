FastComments vodi hostovani Model Context Protocol (MCP) server kako bi AI asistenti i agentski klijenti mogli direktno da pozivaju FastComments API. Svaki alat koji MCP server izlaže automatski se generiše iz javnog OpenAPI specifikacije, tako da sve što REST API može, MCP klijent može.

Endpoint je bezstanjan (stateless) i zasnovan na streamable-HTTP. Ne postoji sesija koju treba održavati niti server-side stanje po klijentu.

### Endpoint

[inline-code-attrs-start title = 'MCP Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Povezivanje putem OAuth-a

Svaki MCP klijent koji podržava udaljene servere sa OAuth-om (Claude, ChatGPT, Claude Code, Cursor i drugi) može se povezati na gore navedeni endpoint bez ikakve konfiguracije na strani FastComments-a. Klijent se registruje putem Dynamic Client Registration-a ili se identifikuje pomoću Client ID Metadata Document-a, otvara pregledač kako biste se prijavili na FastComments i odobrili pristup, i dobija token vezan za nalog na koji ste bili prijavljeni.

Dokumenti za otkrivanje (discovery) nalaze se na standardnim lokacijama:

[inline-code-attrs-start title = 'Discovery'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Vaš korisnik treba da ima API Admin dozvolu na nalogu kako bi odobrio vezu. Ako upravljate više naloga, prebacite se na pravi u kontrolnoj tabli pre odobravanja.

Klijent može zahtevati `read` scope, `write` scope ili oba. Klijent koji ne zahteva ništa dobija oba. Alati koji menjaju podaci se ne nude read-only tokenu.

Kontrolna tabla ima pomoćnika za podešavanje sa gotovim isečcima koda spremnim za lepljenje. Otvorite **Integrate -> MCP Server**, ili posetite direktno:

[inline-code-attrs-start title = 'Stranica za podešavanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registrujte FastComments server jednom komandom, a zatim pokrenite `/mcp` unutar sesije da se prijavite i izlistate dostupne alate:

[inline-code-attrs-start title = 'Claude Code podešavanje'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor i drugi klijenti sa konfiguracionim fajlom

Dodajte ovaj blok u MCP servers konfiguraciju vašeg klijenta (`mcp.json` za Cursor). Klijent otvara pregledač za prijavu pri prvom korišćenju.

[inline-code-attrs-start title = 'MCP klijent konfiguracija'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Opoziv pristupa

Svaka odobrena veza je izlistana pod **Integrate -> Connected Apps** u kontrolnoj tabli. Opoziv jedne poništava svaki token koji ta aplikacija drži. Aplikacije se same registruju prilikom povezivanja i FastComments ih ne pregleda, pa opozovite sve što ne prepoznajete.

### Korišćenje tokena sa REST API-jem

Access token koji MCP klijent dobije je običan FastComments API kredencijal. Radi na svakom `/api/v1` endpointu kao bearer token, tako da aplikacija koja se povezala putem MCP-a može takođe direktno pozivati REST API:

[inline-code-attrs-start title = 'Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Tenant je podrazumevan iz tokena. `tenantId` se i dalje može proslediti ali mora da se poklapa. `GET` zahtevi zahtevaju `read` scope a sve ostalo `write` scope.

### Povezivanje sa API ključem

Klijenti koji ne mogu da završe prijavu kroz pregledač, kao što su headless serveri, mogu se autentifikovati API ključem umesto toga. Prosledite `tenantId` i `API_KEY` kao query parametre, ili kao `x-tenant-id` i `x-api-key` HTTP header-e ako vaš klijent podržava prilagođene header-e:

[inline-code-attrs-start title = 'Endpoint sa API ključem'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Stranica za podešavanje generiše ovaj URL za svaki od vaših API ključeva.

### Bezbednost

URL endpointa koji sadrži API ključ je tajna: ne lepite ga u javne razgovore, screenshot-ove ili commit-ove. Ako je ključ izložen, rotirajte ga na API Keys stranici u vašoj kontrolnoj tabli. OAuth tokeni ne nose takav rizik jer su vezani za jednu aplikaciju i mogu se opozvati iz Connected Apps.