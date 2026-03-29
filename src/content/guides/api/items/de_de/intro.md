### Die FastComments API

FastComments stellt eine API zur Interaktion mit vielen Ressourcen bereit. Erstellen Sie Integrationen mit unserer Plattform oder bauen Sie sogar eigene Clients!

In dieser Dokumentation finden Sie alle vom API unterstützten Ressourcen, dokumentiert mit ihren Anfrage- und Antworttypen.

Für Enterprise-Kunden wird sämtlicher API-Zugriff im Audit-Log erfasst.

### Generierte SDKs

FastComments erstellt jetzt eine [API-Spezifikation](https://fastcomments.com/js/swagger.json) aus unserem Code (dies ist noch nicht vollständig, enthält aber bereits viele APIs).

Wir bieten jetzt auch SDKs für beliebte Sprachen an:

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

Die API wird authentifiziert, indem Sie Ihren [API-Schlüssel](https://fastcomments.com/auth/my-account/api-secret) entweder als `X-API-KEY`-Header oder als `API_KEY`-Query-Parameter übergeben. Sie benötigen außerdem Ihre `tenantId`, um API-Aufrufe zu tätigen. Diese kann von derselben Seite wie Ihr API-Schlüssel abgerufen werden.

### Sicherheitshinweis

Diese Routen sind dafür vorgesehen, von einem **Server** aufgerufen zu werden. __RUFEN SIE SIE NICHT__ von einem Browser auf. Andernfalls würden Sie Ihren API-Schlüssel offenlegen – damit hätte jede Person, die den Quellcode einer Seite einsehen kann, vollen Zugriff auf Ihr Konto!

#### Authentifizierungsoption Eins - Header

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentifizierungsoption Zwei - Query-Parameter

- Query Param: `API_KEY`
- Query Param: `tenantId`

### Eigene Schreibvorgänge lesen

FastComments bietet Active-Active-Verfügbarkeit. Anfragen von Ihrem Rechenzentrum werden an [den nächstgelegenen Point of Presence](https://sophon.fastcomments.com/) zu Ihrem weitergeleitet. Dies geschieht automatisch, und normalerweise können Sie read-your-write semantics beobachten. Wenn Sie sicherstellen möchten, dass Sie Ihre eigenen Schreibvorgänge lesen, können Sie Ihre Anfragen an eine bestimmte Region binden, indem Sie diese Region als API-Host verwenden (dies ist für die meisten Integrationen jedoch in der Regel nicht notwendig):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Beachten Sie, dass Sie in diesem Fall möglicherweise einen Fallback definieren sollten, da wir in der Vergangenheit Entrypoint-Knoten veraltet haben und für den Switchover neue Namen verwenden.