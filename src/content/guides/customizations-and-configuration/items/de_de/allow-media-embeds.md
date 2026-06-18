Standardmäßig erlaubt FastComments keine iframes in Kommentaren. Wenn Sie Medieneinbettungen aktivieren, können Kommentierende den Einbettungscode (das `<iframe>`-Snippet) von vertrauenswürdigen Anbietern wie YouTube, Vimeo, SoundCloud und Spotify einfügen, und dieser wird inline im Kommentar dargestellt.

Aus Sicherheitsgründen ist dies kein clientseitiger Widget-Konfigurationsschalter. Es handelt sich um eine serverseitige Einstellung, die beim Speichern jedes Kommentars validiert wird, sodass sie nicht von der Seite aus eingeschaltet werden kann. Nur iframes, die auf eine integrierte Liste vertrauenswürdiger Anbieter zeigen, sind erlaubt. Alle anderen iframes werden entfernt.

Dies wird ohne Code auf der Seite zur Anpassung des Widgets durchgeführt:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Medieneinbettungen erlauben' app-screenshot-end]

### Eigene Anbieter hinzufügen

Wenn Sie Einbettungen von einem Anbieter zulassen möchten, der nicht auf der integrierten Liste vertrauenswürdiger Anbieter steht, fügen Sie dessen Hostnamen in das Feld "Zusätzliche Einbettungsdomänen" auf derselben Seite ein. Diese Hostnamen sind zusätzlich zu den integrierten Anbietern erlaubt. Die Übereinstimmung ist exakt, geben Sie also den vollständigen Hostnamen an (zum Beispiel player.example.com). Alles, was Sie nicht auflisten, bleibt blockiert.

Sowohl das einfache Kommentarfeld als auch der WYSIWYG-Editor unterstützen das Einfügen einer Einbettung. Im WYSIWYG-Editor wird die Einbettung als entfernbarer Block eingefügt.

---