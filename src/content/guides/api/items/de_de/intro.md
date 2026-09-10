### Die FastComments API

FastComments stellt eine API zur Interaktion mit vielen Ressourcen bereit. Erstellen Sie Integrationen mit unserer Plattform oder sogar Ihre eigenen Clients!

In dieser Dokumentation finden Sie alle von der API unterstützten Ressourcen, dokumentiert mit ihren Anforderungs- und Antworttypen.

Für Enterprise‑Kunden wird jeder API‑Zugriff im Audit‑Log erfasst.

### Generierte SDKs

FastComments generiert jetzt ein [API Spec](https://fastcomments.com/js/swagger.json) aus unserem Code (dies ist noch nicht vollständig, enthält aber viele APIs).

Wir haben jetzt auch SDKs für gängige Programmiersprachen:

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

### Authentifizierung

Die API wird authentifiziert, indem Sie Ihren [api key](https://fastcomments.com/auth/my-account/api-secret) entweder als `X-API-KEY`‑Header oder als `API_KEY`‑Abfrageparameter übergeben. Sie benötigen außerdem Ihre `tenantId` für API‑Aufrufe. Diese kann von derselben Seite wie Ihr API‑Schlüssel abgerufen werden.

### Sicherheits‑Hinweis

Diese Routen sind dafür gedacht, von einem **Server** aus aufgerufen zu werden. __NICHT__ von einem Browser aus aufrufen. Dadurch wird Ihr API‑Schlüssel offengelegt – dies gewährt jedem, der den Quellcode einer Seite einsehen kann, vollen Zugriff auf Ihr Konto!

#### Authentifizierungsoption Eins – Header

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentifizierungsoption Zwei – Abfrageparameter

- Abfrageparameter: `API_KEY`
- Abfrageparameter: `tenantId`

#### Authentifizierungsoption Drei – OAuth‑Bearer‑Token

- Header: `Authorization: Bearer fcat_...`

Anwendungen, die über den [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) verbunden werden, erhalten ein Token über OAuth anstelle eines API‑Schlüssels. Dieses Token funktioniert bei allen Endpunkten hier. Der Mandant ist im Token impliziert, sodass `tenantId` optional ist, aber bei Angabe mit dem Token übereinstimmen muss. `GET`‑Anfragen benötigen den `read`‑Scope und alle anderen Methoden den `write`‑Scope. Die Entdeckung beginnt bei `https://fastcomments.com/.well-known/oauth-authorization-server`.

### Lesen Ihrer eigenen Schreibvorgänge

FastComments bietet Active‑Active‑Verfügbarkeit. Anfragen aus Ihrem Rechenzentrum werden zu dem [nächsten Point of Presence](https://sophon.fastcomments.com/) zu Ihnen geleitet. Dies geschieht automatisch, und normalerweise können Sie die Lese‑nach‑Schreiben‑Semantik beobachten. Wenn Sie sicherstellen möchten, dass Sie Ihre eigenen Schreibvorgänge lesen, können Sie Ihre Anfragen an eine bestimmte Region binden, indem Sie diese Region als API‑Host verwenden (dies ist jedoch für die meisten Integrationen normalerweise nicht erforderlich):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Beachten Sie, dass Sie in diesem Fall möglicherweise ein Fallback definieren möchten, da wir in der Vergangenheit Einstiegspunkte eingestellt haben und neue Namen für den Umschaltvorgang verwenden.