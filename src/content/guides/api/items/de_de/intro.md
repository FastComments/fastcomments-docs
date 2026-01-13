### Die FastComments API

FastComments stellt eine API zur Verfügung, um mit vielen Ressourcen zu interagieren. Erstellen Sie Integrationen mit unserer Plattform oder bauen Sie sogar Ihre eigenen Clients!

In dieser Dokumentation finden Sie alle vom API unterstützten Ressourcen, dokumentiert mit ihren Anfrage- und Antworttypen.

Für Enterprise-Kunden wird sämtlicher API-Zugriff im Audit-Log protokolliert.

### Generierte SDKs

FastComments erzeugt jetzt eine [API-Spezifikation](https://fastcomments.com/js/swagger.json) aus unserem Code (dies ist noch nicht vollständig, enthält aber viele APIs).

Wir haben jetzt auch SDKs für beliebte Sprachen:

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

Die API wird authentifiziert, indem Sie Ihren [api key](https://fastcomments.com/auth/my-account/api-secret) entweder als `X-API-KEY` Header oder als `API_KEY` Query-Parameter übergeben. Sie benötigen außerdem Ihre `tenantId`
für API-Aufrufe. Diese kann auf derselben Seite wie Ihr api key abgerufen werden.

### Sicherheitshinweis

Diese Routen sind dafür gedacht, von einem **Server** aufgerufen zu werden. __RUFEN SIE SIE NICHT__ aus einem Browser auf. Wenn Sie dies tun, wird Ihr API-Schlüssel offengelegt - dadurch erhält jede Person, die den Quellcode einer Seite einsehen kann, vollen Zugriff auf Ihr Konto!

#### Authentifizierungsoption Eins - Header

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentifizierungsoption Zwei - Query-Parameter

- Query-Parameter: `API_KEY`
- Query-Parameter: `tenantId`