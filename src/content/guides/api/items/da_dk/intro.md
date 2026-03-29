### FastComments API

FastComments leverer et API til at interagere med mange ressourcer. Byg integrationer med vores platform, eller byg endda dine egne klienter!

I denne dokumentation finder du alle de ressourcer, som API'et understøtter, dokumenteret med deres request- og response-typer.

For Enterprise-kunder logges al API-adgang i Audit Log.

### Generated SDKs

FastComments genererer nu en [API Spec](https://fastcomments.com/js/swagger.json) fra vores kode (denne er endnu ikke komplet, men inkluderer mange API'er).

Vi har nu også SDK'er til populære sprog:

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

### Authentication

API'et autentificeres ved at sende din [api key](https://fastcomments.com/auth/my-account/api-secret) enten som en `X-API-KEY` header eller som `API_KEY` query-parameter. Du skal også bruge din `tenantId` for at foretage API-kald. Dette kan hentes fra samme side som din api key.

### Security Note

Disse ruter er beregnet til at blive kaldt fra en **server**. __DO NOT__ kald dem fra en browser. Hvis du gør det, vil det udsætte din API key - det vil give fuld adgang til din konto til enhver, der kan se kildeteksten på en side!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Reading Your Own Writes

FastComments tilbyder Active-Active-tilgængelighed. Anmodninger fra dit datacenter dirigeres til [det nærmeste point of presence](https://sophon.fastcomments.com/) i forhold til dit. Dette er automatisk, og normalt kan du observere read-your-write semantics. Hvis du vil være sikker på at kunne læse dine egne ændringer, kan du fastgøre dine anmodninger til en bestemt region ved at bruge den region som dens API-vært (det er dog normalt ikke nødvendigt for de fleste integrationer):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Bemærk, at hvis du gør dette, kan du få brug for at definere en fallback, da vi tidligere har udfaset entrypoint-noder og brugt nye navne ved overgangen.