Standardmäßig erlaubt FastComments keine iframes in Kommentaren. Wenn Sie Media-Embeds aktivieren, können Kommentierende den Embed-Code (das `<iframe>`-Snippet) von vertrauenswürdigen Anbietern wie YouTube, Vimeo, SoundCloud und Spotify einfügen, und er wird inline im Kommentar gerendert.

Aus Sicherheitsgründen ist dies kein clientseitiges Widget-Konfigurationsflag. Es ist eine serverseitige Einstellung, die beim Speichern jedes Kommentars validiert wird, sodass sie nicht von der Seite aus aktiviert werden kann. Nur iframes, die auf eine integrierte Liste vertrauenswürdiger Anbieter verweisen, sind erlaubt. Jeder andere iframe wird entfernt.

Dies geschieht ohne Code, auf der Widget-Anpassungsseite:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Einstellung für Media-Embeds auf der Widget-Anpassungsseite aktiviert, sodass Kommentierende vertrauenswürdige iframe-Embeds einfügen können'; title='Media-Embeds erlauben' app-screenshot-end]

### Eigene Anbieter hinzufügen

Wenn Sie Embeds von einem Anbieter erlauben möchten, der nicht auf der integrierten Vertrauensliste steht, fügen Sie seinen Hostnamen in das Feld „Additional Embed Domains" auf derselben Seite ein. Diese Hostnamen werden zusätzlich zu den integrierten Anbietern erlaubt. Die Übereinstimmung ist exakt, geben Sie also den vollständigen Hostnamen an (zum Beispiel player.example.com). Alles, was Sie nicht auflisten, bleibt blockiert.

Sowohl das einfache Kommentarfeld als auch der WYSIWYG-Editor unterstützen das Einfügen eines Embeds. Im WYSIWYG-Editor wird der Embed als entfernbarer Block eingefügt.