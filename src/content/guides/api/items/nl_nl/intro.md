### De FastComments API

FastComments biedt een API voor het werken met veel resources. Bouw integraties met ons platform, of zelfs je eigen clients!

In deze documentatie vind je alle door de API ondersteunde resources gedocumenteerd met hun request- en response-typen.

Voor Enterprise-klanten wordt alle API-toegang vastgelegd in het Audit Log.

### Generated SDKs

FastComments genereert nu een [API Spec](https://fastcomments.com/js/swagger.json) uit onze code (dit is nog niet compleet, maar bevat al veel APIs).

We hebben nu ook SDKs voor populaire talen:

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

De API wordt geauthenticeerd door je [API-sleutel](https://fastcomments.com/auth/my-account/api-secret) mee te geven als ofwel een `X-API-KEY` header of als `API_KEY` query parameter. Je hebt ook je `tenantId` nodig om API-aanroepen te doen. Deze is op dezelfde pagina als je API-sleutel terug te vinden.

### Beveiligingsnotitie

Deze routes zijn bedoeld om vanaf een **server** aangeroepen te worden. __DO NOT__ roep ze niet vanaf een browser aan. Dat zal je API-sleutel blootstellen — hiermee krijgt iedereen die de broncode van een pagina kan bekijken volledige toegang tot je account!

#### Authenticatie-optie één - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authenticatie-optie twee - Query-parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Je eigen schrijfbewerkingen lezen

FastComments biedt Active-Active beschikbaarheid. Verzoeken van je datacenter worden gerouteerd naar [het dichtstbijzijnde point of presence](https://sophon.fastcomments.com/) bij jou. Dit is automatisch, en normaal gesproken kun je de read-your-write-semantiek waarnemen. Als je er zeker van wilt zijn dat je je eigen schrijfbewerkingen leest, kun je je verzoeken vastzetten op een bepaald gebied door dat gebied als API-host te gebruiken (dit is echter meestal niet nodig voor de meeste integraties):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Houd er rekening mee dat als je dit doet je mogelijk een terugvaloptie wilt definiëren, omdat we in het verleden entrypoint-nodes hebben afgeschaft en nieuwe namen gebruiken voor de overschakeling.