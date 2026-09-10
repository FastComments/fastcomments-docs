### De FastComments API

FastComments biedt een API voor interactie met vele resources. Bouw integraties met ons platform, of zelfs je eigen clients!

In deze documentatie vind je alle ondersteunde resources van de API, gedocumenteerd met hun aanvraag- en responstypen.

Voor Enterprise-klanten wordt alle API-toegang vastgelegd in het Audit Log.

### Gegenereerde SDK's

FastComments genereert nu een [API Spec](https://fastcomments.com/js/swagger.json) vanuit onze code (dit is nog niet volledig, maar bevat veel API's).

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

De API wordt geauthenticeerd door je [api key](https://fastcomments.com/auth/my-account/api-secret) mee te geven als een `X-API-KEY` header of `API_KEY` query‑parameter. Je hebt ook je `tenantId` nodig voor het doen van API‑aanroepen. Deze kan worden opgehaald vanaf dezelfde pagina als je api‑key.

### Beveiligingsopmerking

Deze routes zijn bedoeld om vanaf een **server** te worden aangeroepen. __ROEP ZE NIET__ aan vanuit een browser. Dit zou je API‑key blootleggen – dit geeft volledige toegang tot je account aan iedereen die de broncode van een pagina kan bekijken!

#### Authenticatieoptie één - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authenticatieoptie twee - Queryparameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### Authenticatieoptie drie - OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Applicaties die via de [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) verbinden, verkrijgen een token via OAuth in plaats van een API‑key. Dat token werkt op elk endpoint hier. De tenant wordt geïmpliceerd door het token, dus `tenantId` is optioneel, maar moet overeenkomen met het token wanneer het wordt opgegeven. `GET`‑verzoeken hebben de `read`‑scope nodig en elke andere methode heeft de `write`‑scope nodig. Ontdekking start op `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Je eigen schrijfbewerkingen lezen

FastComments biedt Active‑Active beschikbaarheid. Verzoeken vanuit je datacenter worden gerouteerd naar [het dichtstbijzijnde point of presence](https://sophon.fastcomments.com/) van jou. Dit gebeurt automatisch, en normaal kun je de read‑your‑write‑semantiek observeren. Als je er zeker van wilt zijn dat je je eigen schrijfbewerkingen leest, kun je je verzoeken vastzetten op een bepaalde regio door die regio als API‑host te gebruiken (hoewel dit meestal niet nodig is voor de meeste integraties):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Let op dat je, als je dit doet, mogelijk een fallback wilt definiëren, aangezien we in het verleden entrypoint‑nodes hebben uitgefaseerd en nieuwe namen gebruiken voor de overschakeling.