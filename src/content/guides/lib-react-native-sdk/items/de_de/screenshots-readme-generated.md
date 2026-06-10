#### Design: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Design: Standard
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativer WYSIWYG-Editor mit Bildunterstützung!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rich-Text-Editor

Diese Bibliothek verwendet [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) für die Rich-Text-Bearbeitung, welche eine leistungsstarke WYSIWYG-Bearbeitungserfahrung bietet. Derselbe Editor treibt iOS, Android und das Web (via `react-native-web`) an, sodass der Composer auf allen Plattformen mit einer einzigen Implementierung konsistent funktioniert.

`react-native-enriched` erfordert die React Native New Architecture (Fabric) auf nativen Plattformen sowie einen Bundler, der die Package-`exports`-Bedingungen auflöst (Metro mit Package-Exports / RN 0.72+). Web-Unterstützung ist derzeit experimentell.

### Konfigurationsoptionen

Diese Bibliothek hat das Ziel, alle in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) definierten Konfigurationsoptionen zu unterstützen, genau wie die Web-Implementierung.

### FastComments-Konzepte

Die wichtigsten Konzepte, die man zum Einstieg kennen sollte, sind `tenantId` und `urlId`. `tenantId` ist Ihre FastComments.com-Kontokennung. `urlId` ist der Ort, an den Kommentar-Threads gebunden werden. Dies kann eine Seiten-URL, eine Produkt-ID, eine Artikel-ID usw. sein.

### Benutzerbenachrichtigungen

FastComments unterstützt Benachrichtigungen für [viele Szenarien](https://docs.fastcomments.com/guide-notifications.html). Benachrichtigungen sind konfigurierbar, können global oder auf Ebene einer einzelnen Benachrichtigung/eines Kommentars abgewählt werden und unterstützen Seiten-Abonnements, sodass Benutzer Threads einer bestimmten Seite oder eines bestimmten Artikels abonnieren können.

Zum Beispiel ist es möglich, Secure SSO zur Authentifizierung des Benutzers zu verwenden und dann periodisch auf ungelesene Benachrichtigungen zu prüfen und diese an den Benutzer zu übermitteln.

Siehe [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) dafür, wie man ungelesene Benutzerbenachrichtigungen abruft und übersetzt.

### Gif-Browser

Standardmäßig ist keine Auswahl von Bildern oder GIFs aktiviert. Siehe [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dafür, wie man Bild- und GIF-Uploads unterstützt. Es gibt einen Gif-Browser in dieser Bibliothek, der Suchanfragen und bereitgestellte Bilder anonymisiert — Sie müssen ihn nur verwenden.

### Leistung

Bitte eröffnen Sie ein Ticket mit einem reproduzierbaren Beispiel, inklusive des verwendeten Geräts, wenn Sie Leistungsprobleme feststellen. Leistung hat in allen FastComments-Bibliotheken oberste Priorität.