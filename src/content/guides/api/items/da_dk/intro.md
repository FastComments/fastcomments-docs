### FastComments API'en

FastComments leverer et API til at interagere med mange ressourcer. Byg integrationer med vores platform, eller endda byg dine egne klienter!

I denne dokumentation finder du alle understøttede ressourcer i API'et dokumenteret med deres anmodnings- og svar-typer.

For Enterprise-kunder registreres al API-adgang i revisionsloggen.

### Genererede SDK'er

FastComments genererer nu et [API Spec](https://fastcomments.com/js/swagger.json) fra vores kode (dette er endnu ikke komplet, men inkluderer mange API'er).

Vi har også nu SDK'er for populære sprog:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Godkendelse

API'et autentificeres ved at sende din [api-nøgle](https://fastcomments.com/auth/my-account/api-secret) som enten en `X-API-KEY` header eller `API_KEY` query-parameter. Du har også brug for din `tenantId` for at foretage API-kald. Denne kan hentes fra den samme side som din api-nøgle.

### Sikkerhedsnote

Disse ruter er beregnet til at blive kaldt fra en **server**. __KALL IKKE__ dem fra en browser. At gøre det vil afsløre din API-nøgle - dette vil give fuld adgang til din konto til enhver, der kan se kildekoden på en side!

#### Godkendelsesmulighed En - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Godkendelsesmulighed To - Query Parametre

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Godkendelsesmulighed Tre - OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Tredjepartsapplikationer såsom Zapier og klienter af [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) får en token via OAuth i stedet for en API-nøgle. Den token fungerer på alle endpoints her. Lejeren er underforstået af tokenen, så `tenantId` er valgfri, men den skal matche tokenen, hvis den angives. `GET`-anmodninger kræver `read`-scopet og alle andre metoder kræver `write`-scopet. Den fulde flow, inklusive klientregistrering, PKCE, opdatering og tilbagekaldelse, er dokumenteret under [OAuth Authorization](#oauth). Opdagelse starter på `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Læsning af Dine Egne Skrivninger

FastComments leverer Active-Active tilgængelighed. Anmodninger fra dit datacenter dirigeres til [det nærmeste tilstedeværelsespunkt](https://sophon.fastcomments.com/) i forhold til dit. Dette er automatisk, og normalt kan du observere læs-dine-skriv-semantik. Hvis du vil være sikker på at læse dine egne skrivninger, kan du fastgøre dine anmodninger til en bestemt region ved at bruge den region som API-vært (men dette er normalt ikke nødvendigt for de fleste integrationer):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Bemærk, at hvis du gør dette, vil du måske definere en fallback, da vi har afviklet indgangspunkt-noder tidligere og bruger nye navne til overgangen.