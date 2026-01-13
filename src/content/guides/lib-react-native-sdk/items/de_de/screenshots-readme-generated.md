#### Skin: Erebus
![Skin: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Skin: Default
![Skin: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Native WYSIWYG Editor with Image Support!
![Native WYSIWYG Editor with Image Support](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Rich Text Editor

Diese Bibliothek verwendet den 10tap-Editor für Rich-Text-Bearbeitungsfunktionen, der ein leistungsfähiges WYSIWYG-Bearbeitungserlebnis bietet.

### Configuration Options

Diese Bibliothek hat zum Ziel, alle Konfigurationsoptionen zu unterstützen, die in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) definiert sind, genau wie die Web-Implementierung.

### FastComments Concepts

Die wichtigsten Konzepte, die Sie zum Einstieg kennen sollten, sind `tenantId` und `urlId`. `tenantId` ist Ihre FastComments.com-Kontoidentifikation. `urlId` ist der Ort, an den Kommentarthreads gebunden werden. Dies kann eine Seiten-URL, eine Produkt-ID, eine Artikel-ID usw. sein.

### User Notifications

FastComments unterstützt Benachrichtigungen für [viele Szenarien](https://docs.fastcomments.com/guide-notifications.html). Benachrichtigungen sind konfigurierbar, können global oder auf Benachrichtigungs-/Kommentar-Ebene abgewählt werden und unterstützen Seiten-Abonnements, sodass sich Benutzer für die Threads einer bestimmten Seite oder eines bestimmten Artikels anmelden können.

Beispielsweise ist es möglich, Secure SSO zur Authentifizierung des Benutzers zu verwenden und dann periodisch nach ungelesenen Benachrichtigungen zu fragen und diese an den Benutzer zu senden.

Siehe [das Beispiel AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) dafür, wie man ungelesene Benutzerbenachrichtigungen abruft und übersetzt.

### Gif Browser

Standardmäßig sind keine Bild- oder GIF-Auswahlen aktiviert. Siehe [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) dafür, wie man Bild- und GIF-Uploads unterstützt. In dieser Bibliothek gibt es einen GIF-Browser, der Suchanfragen und Bilder anonymisiert bereitstellt — Sie müssen ihn nur verwenden.

### Performance

Bitte eröffnen Sie ein Ticket mit einem reproduzierbaren Beispiel, einschließlich des verwendeten Geräts, wenn Sie Leistungsprobleme feststellen. Performance hat in allen FastComments-Bibliotheken oberste Priorität.