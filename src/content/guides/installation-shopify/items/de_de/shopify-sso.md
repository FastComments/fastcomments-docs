Der **FastComments**-Block unterstützt Single Sign-On (SSO), sodass Ihre Shopify-Kund:innen als sich selbst kommentieren können, ohne ein separates FastComments-Konto erstellen zu müssen.

### Wie es funktioniert

Wenn ein Besucher, der in Ihrem Shop eingeloggt ist, eine Seite mit dem **FastComments**-Block öffnet:

1. Der Block erkennt das Shopify-`customer`-Objekt.
2. Er sendet den Namen und die E-Mail des Kunden über eine signierte App-Proxy-Anfrage an FastComments.
3. FastComments erstellt oder findet einen Nutzer mit dem Schlüssel `shopify-{customerId}`, sodass derselbe Kunde über Sitzungen und Neuinstallationen hinweg immer demselben FastComments-Nutzer zugeordnet wird.
4. Der Name des Besuchers wird bei seinen Kommentaren angezeigt. Er wird nicht erneut zur Anmeldung aufgefordert.

Wenn der Besucher nicht in den Shop eingeloggt ist, fällt der Block auf anonyme Kommentare zurück (oder auf den FastComments-Anmeldeablauf, abhängig von Ihrer Widget-Konfiguration).

### SSO deaktivieren

SSO ist für jeden **FastComments**-Block standardmäßig aktiviert. Um es für einen bestimmten Block auszuschalten:

1. Öffnen Sie den Shopify-Theme-Editor.
2. Öffnen Sie die Vorlage, die den Block enthält, und klicken Sie auf den Block, um ihn auszuwählen.
3. Deaktivieren Sie **SSO**.
4. Klicken Sie auf **Speichern**.

Deaktivieren Sie SSO, wenn Sie möchten, dass Kommentierende eine separate Identität für die Konversation wählen können. Zum Beispiel eine interne Community-Seite, auf der Mitarbeitende unter einem anderen Anzeigenamen kommentieren.

### Was FastComments erhält

Die für jeden Kunden gesendete SSO-Nutzlast enthält:

- Eine Benutzer-ID, die aus der Shopify-Kunden-ID abgeleitet ist (`shopify-{customerId}`).
- Die E-Mail des Kunden (wird zur Identifizierung des Nutzers verwendet; nicht öffentlich angezeigt).
- Den Anzeigenamen des Kunden (wird als Name des Kommentarautors verwendet).

Es werden keine Bestellhistorie-, Zahlungs- oder Adressdaten gesendet. Die Nutzlast wird serverseitig signiert; der Browser des Kunden sieht niemals Anmeldeinformationen.

### Anmelde- und Abmeldelinks

Wenn SSO aktiviert ist, zeigen die Anmelde- und Abmeldelinks des Kommentar-Widgets auf `/account/login` und `/account/logout`, die standardmäßigen Shopify-Routen für Kundenkonten. Es ist nichts zu konfigurieren. Die Links funktionieren für jeden Shop, bei dem Kundenkonten aktiviert sind.