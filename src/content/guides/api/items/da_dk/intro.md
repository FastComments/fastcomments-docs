### FastComments API

FastComments tilbyder et API til at interagere med mange ressourcer. Byg integrationer med vores platform, eller byg endda dine egne klienter!

I denne dokumentation finder du alle API'ens understøttede ressourcer dokumenteret med deres request- og response-typer.

For Enterprise-kunder registreres al API-adgang i Audit Log.

### Genererede SDK'er

FastComments genererer nu en [API Spec](https://fastcomments.com/js/swagger.json) fra vores kode (dette er endnu ikke fuldt færdigt, men omfatter mange API'er).

Vi har også nu SDK'er til populære sprog:

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

### Autentificering

API'en autentificeres ved at videregive din [API-nøgle](https://fastcomments.com/auth/my-account/api-secret) enten som en `X-API-KEY` header eller som forespørgselsparameteren `API_KEY`. Du får også brug for din `tenantId` for at foretage API-opkald. Den kan hentes fra samme side som din API-nøgle.

### Sikkerhedsnote

Disse ruter er beregnet til at blive kaldt fra en **server**. __KALD IKKE__ dem fra en browser. Hvis du gør det, vil din API-nøgle blive eksponeret - det vil give fuld adgang til din konto for enhver, der kan se kildeteksten på en side!

#### Autentificeringsmulighed Én - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Autentificeringsmulighed To - Forespørgselsparametre

- Forespørgselsparameter: `API_KEY`
- Forespørgselsparameter: `tenantId`

---