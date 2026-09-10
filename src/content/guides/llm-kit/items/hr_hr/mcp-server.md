FastComments pokreće hostani Model Context Protocol (MCP) poslužitelj kako bi AI asistenti i agentni klijenti mogli izravno pozivati FastComments API. Svaki alat koji MCP poslužitelj izlaže automatski je generiran iz javne OpenAPI specifikacije, pa sve što REST API može učiniti, MCP klijent također može učiniti.

Krajnja točka je bez stanja i temeljena na streamable-HTTP. Nema sesije koja se mora održavati i nema server‑side stanja po klijentu.

### Endpoint

[inline-code-attrs-start title = 'MCP krajnja točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Bilo koji MCP klijent koji podržava udaljene poslužitelje s OAuth-om (Claude, ChatGPT, Claude Code, Cursor i drugi) može se povezati na gore navedenu krajnju točku bez ikakve postavke na FastComments strani. Klijent se registrira putem Dinamičke registracije klijenta ili se identificira pomoću Dokumenta metapodataka ID‑klijenta, otvara preglednik kako biste se mogli prijaviti u FastComments i odobriti pristup, te prima token vezan uz račun na koji ste prijavljeni.

Dokumenti za otkrivanje nalaze se na standardnim lokacijama:

[inline-code-attrs-start title = 'Otkrivanje'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Vaš korisnik treba imati dozvolu API Admin na računu kako bi odobrio vezu. Ako upravljate s više računa, prije odobravanja prebacite se na pravi račun u nadzornoj ploči.

Klijent može zatražiti `read` opseg, `write` opseg ili oba. Klijent koji ne zatraži ništa dobiva oba. Alati koji mijenjaju podatke nisu dostupni tokenu s pravom samo za čitanje.

Nadzorna ploča ima pomoćnik za postavljanje s isječcima spremnim za umetanje. Otvorite **Integrate -> MCP Server**, ili ga posjetite izravno:

[inline-code-attrs-start title = 'Stranica postavki'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Registrirajte FastComments poslužitelj jednim naredbom, zatim pokrenite `/mcp` unutar sesije za prijavu i popis dostupnih alata:

[inline-code-attrs-start title = 'Postavljanje Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Dodajte ovaj blok u konfiguraciju MCP poslužitelja vašeg klijenta (`mcp.json` za Cursor). Klijent otvara preglednik za prijavu pri prvom korištenju.

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

Svaka odobrena veza nalazi se pod **Integrate -> Connected Apps** u nadzornoj ploči. Povlačenjem jedne veze poništava se svaki token koji ta aplikacija drži. Aplikacije se registriraju kada se povežu, a FastComments ih ne pregledava, stoga povucite sve što ne prepoznajete.

### Using the token with the REST API

Pristupni token koji MCP klijent dobije je regularna FastComments API vjerodajnica. Djeluje na svakoj `/api/v1` krajnjoj točki kao token nositelja, pa aplikacija koja se poveže putem MCP-a također može izravno pozivati REST API:

[inline-code-attrs-start title = 'Token nositelja'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Najmodavac (tenant) je impliciran tokenom. `tenantId` se i dalje može proslijediti, ali mora se podudarati. `GET` zahtjevi trebaju `read` opseg, a sve ostalo `write` opseg.

### Connect with an API key

Klijenti koji ne mogu dovršiti prijavu putem preglednika, poput headless poslužitelja, mogu se autentificirati pomoću API ključa. Proslijedite `tenantId` i `API_KEY` kao parametre upita, ili kao HTTP zaglavlja `x-tenant-id` i `x-api-key` ako vaš klijent podržava prilagođena zaglavlja:

[inline-code-attrs-start title = 'Krajnja točka API ključa'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Stranica postavki generira ovaj URL za svaki od vaših API ključeva.

### Security

URL krajnje točke koji sadrži API ključ je tajna: nemojte ga zalijepiti u javne razgovore, snimke zaslona ili commitove. Ako je ključ otkriven, zamijenite ga na stranici API ključeva u vašoj nadzornoj ploči. OAuth tokeni ne nose takav rizik jer su vezani uz jednu aplikaciju i mogu se povući iz Connected Apps.

---