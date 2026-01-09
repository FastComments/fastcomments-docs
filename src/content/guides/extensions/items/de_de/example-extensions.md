Bei FastComments schreiben wir unsere eigenen Erweiterungen, die dieselbe API verwenden. Sie können den unminifizierten Code
für diese Erweiterungen unter den folgenden Endpunkten einsehen:

#### Dunkelmodus

Die Dunkelmodus-Erweiterung wird bedingt geladen, wenn eine „dunkle“ Seite erkannt wird. Diese Erweiterung fügt dem Kommentar-Widget einfach etwas CSS hinzu. Auf diese Weise müssen wir das CSS für den Dunkelmodus nicht laden, wenn es nicht benötigt wird.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderation

Wenn der aktuelle Benutzer ein Moderator ist, verwenden wir die Moderations-Erweiterung.

Dies ist ein gutes Beispiel dafür, klickbasierte Event-Listener hinzuzufügen, API-Anfragen zu stellen sowie Elemente zum Kommentar-Menü und zu Kommentarbereichen hinzuzufügen.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Live-Chat

Die Live-Chat-Erweiterung (in Kombination mit anderer Konfiguration und Gestaltung) verwandelt das Kommentar-Widget in eine Live-Chat-
Komponente.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js