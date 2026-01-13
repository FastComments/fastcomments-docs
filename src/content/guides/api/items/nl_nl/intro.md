### De FastComments API

FastComments biedt een API voor het werken met veel resources. Bouw integraties met ons platform, of bouw zelfs je eigen clients!

In deze documentatie vindt u alle door de API ondersteunde resources, gedocumenteerd met hun request- en response-typen.

Voor Enterprise-klanten wordt alle API-toegang vastgelegd in het Audit Log.

### Gegenereerde SDK's

FastComments genereert nu een [API-specificatie](https://fastcomments.com/js/swagger.json) uit onze code (dit is nog niet compleet, maar bevat veel API's).

We hebben nu ook SDK's voor populaire talen:

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

### Authenticatie

De API wordt geauthenticeerd door uw [API-sleutel](https://fastcomments.com/auth/my-account/api-secret) als `X-API-KEY` header of `API_KEY` queryparameter mee te sturen. U heeft ook uw `tenantId` nodig om API-aanroepen te doen. Deze kan worden opgehaald vanaf dezelfde pagina als uw API-sleutel.

### Beveiligingsnotitie

Deze routes zijn bedoeld om vanaf een **server** te worden aangeroepen. __ROEP ZE NIET__ vanuit een browser. Als u dat doet, maakt u uw API-sleutel openbaar - dit geeft volledige toegang tot uw account aan iedereen die de broncode van een pagina kan bekijken!

#### Authenticatieoptie één - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authenticatieoptie twee - Query-parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

---