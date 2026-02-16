#### Skin: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skin: Default
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Native WYSIWYG Editor with Image Support!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rich Text Editor

Diese Bibliothek verwendet den 10tap editor für Rich-Text-Bearbeitungsfunktionen und bietet damit eine leistungsfähige WYSIWYG-Bearbeitungserfahrung.

### Configuration Options

Diese Bibliothek versucht, alle Konfigurationsoptionen zu unterstützen, die in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) definiert sind, genau wie die Web-Implementierung.

### FastComments Concepts

Die wichtigsten Konzepte, die Sie zum Einstieg kennen sollten, sind `tenantId` und `urlId`. `tenantId` ist Ihr FastComments.com-Kontoidentifikator. `urlId` ist die Zuordnung, an der Kommentar-Threads hängen. Das kann eine Seiten-URL, eine Produkt-ID, eine Artikel-ID usw. sein.

### User Notifications

FastComments unterstützt Benachrichtigungen für [viele Szenarien](https://docs.fastcomments.com/guide-notifications.html). Benachrichtigungen sind konfigurierbar, können global oder auf Ebene einer Benachrichtigung/eines Kommentars abgewählt werden und unterstützen Seiten-abonnements, sodass Nutzer Threads einer bestimmten Seite oder eines Artikels abonnieren können.

Zum Beispiel ist es möglich, Secure SSO zur Authentifizierung des Benutzers zu verwenden und dann periodisch nach ungelesenen Benachrichtigungen zu fragen und diese an den Benutzer zu pushen.

Siehe [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) dafür, wie ungelesene Benutzerbenachrichtigungen abgerufen und übersetzt werden.

### Gif Browser

Standardmäßig ist keine Bild- oder GIF-Auswahl aktiviert. Siehe [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dafür, wie Bild- und GIF-Uploads unterstützt werden. Es gibt einen GIF-Browser in dieser Bibliothek, der Suchanfragen und bereitgestellte Bilder anonymisiert — Sie müssen ihn nur verwenden.

### Performance

Bitte öffnen Sie ein Ticket mit einem Beispiel zur Reproduzierung, einschließlich des verwendeten Geräts, falls Sie Leistungsprobleme feststellen. Performance hat in allen FastComments-Bibliotheken oberste Priorität.