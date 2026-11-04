FastComments vodi hostirani Model Context Protocol (MCP) poslužitelj kako bi AI asistenti i agentski klijenti mogli izravno pozivati FastComments API. Svaki alat koji MCP poslužitelj izlaže automatski se generira iz javne OpenAPI specifikacije, tako da sve što REST API može učiniti, može učiniti i MCP klijent.

Endpoint je bezstanjski i temelji se na streamable-HTTP. Nema sesije koju treba održavati niti stanja na poslužitelju po klijentu.

### Endpoint

[inline-code-attrs-start title = 'MCP Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Povezivanje putem OAuth-a

Svaki MCP klijent koji podržava udaljene poslužitelje s OAuth-om (Claude, ChatGPT, Claude Code, Cursor i drugi) može se povezati s gore navedenim endpointom bez ikakve postavke na strani FastComments. Klijent se registrira putem Dynamic Client Registration ili se identificira s Client ID Metadata Documentom, otvara preglednik kako biste se prijavili u FastComments i odobrili pristup, te prima token vezan uz račun na koji ste bili prijavljeni.

Dokumenti za otkrivanje nalaze se na standardnim lokacijama:

[inline-code-attrs-start title = 'Otkrivanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Vaš korisnik treba API Admin dozvolu na računu kako bi odobrio vezu. Ako upravljate s više računa, prije odobrenja prebacite se na pravi račun u nadzornoj ploči.

Klijent može zatražiti `read` scope, `write` scope ili oboje. Klijent koji ništa ne zatraži dobiva oboje. Alati koji mijenjaju podatke nisu dostupni tokenima s samo čitanjem.

Nadzorna ploča ima pomoćnika za postavku s gotovim isječcima za lijepljenje. Otvorite **Integrate -> MCP Server** ili ga posjetite izravno:

[inline-code-attrs-start title = 'Stranica za postavku'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registrirajte FastComments poslužitelj jednom naredbom, a zatim pokrenite `/mcp` unutar sesije kako biste se prijavili i prikazali dostupne alate:

[inline-code-attrs-start title = 'Postavka za Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor i drugi klijenti s konfiguracijskom datotekom

Dodajte ovaj blok u MCP konfiguraciju poslužitelja vašeg klijenta (`mcp.json` za Cursor). Klijent otvara preglednik za prijavu pri prvom korištenju.

[inline-code-attrs-start title = 'Konfiguracija MCP klijenta'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Svaka odobrena veza navedena je pod **Integrate -> Connected Apps** u nadzornoj ploči. Opoziv jedne poništava svaki token koji ta aplikacija ima. Aplikacije se same registriraju prilikom povezivanja i FastComments ih ne pregledava, pa opozovite sve što ne prepoznajete.

### Korištenje tokena s REST API-jem

Pristupni token koji MCP klijent dobije je redovna FastComments API vjerodajnica. Radi na svakom `/api/v1` endpointu kao bearer token, tako da aplikacija koja se povezala putem MCP-a može izravno pozivati i REST API:

[inline-code-attrs-start title = 'Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Tenant je impliciran tokenom. `tenantId` se i dalje može proslijediti, ali mora odgovarati. `GET` zahtjevi trebaju `read` scope, a sve ostalo treba `write` scope.

### Povezivanje s API ključem

Klijenti koji ne mogu dovršiti prijavu putem preglednika, poput headless poslužitelja, mogu se autentificirati API ključem. Proslijedite `tenantId` i `API_KEY` kao query parametre, ili kao `x-tenant-id` i `x-api-key` HTTP zaglavlja ako vaš klijent podržava prilagođena zaglavlja:

[inline-code-attrs-start title = 'Endpoint s API ključem'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Stranica za postavku generira ovaj URL za svaki od vaših API ključeva.

### Sigurnost

URL endpointa koji sadrži API ključ je tajna: ne lijepite ga u javne razgovore, snimke zaslona ili commitove. Ako je ključ izložen, rotirajte ga na stranici API Keys u vašoj nadzornoj ploči. OAuth tokeni nemaju takav rizik jer su vezani uz jednu aplikaciju i mogu se opozvati iz Connected Apps.