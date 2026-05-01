Das Top Pages-Widget zeigt die meistkommentierten Seiten Ihrer Website an. Es ist nützlich, um neuen Besuchern Ihre ansprechendsten Inhalte zu präsentieren und die Verweildauer auf der Website zu erhöhen.

## Optionen

- **Titel** (optional): Die Überschrift, die über der Liste angezeigt wird. Standardmäßig: "Top Pages".

Das Top Pages-Widget wählt basierend auf den verfügbaren Daten sein eigenes Layout und akzeptiert kein 'count'-Attribut.

## So fügen Sie es hinzu

### Innerhalb eines Beitrags oder einer Seite

Fügen Sie im Block-Editor einen **Shortcode**-Block hinzu und fügen Sie folgendes ein:

[inline-code-attrs-start title = 'Top-Seiten-Shortcode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### In einer Seitenleiste oder Fußzeile (klassische Themes)

Gehen Sie in Ihrer WordPress-Admin-Oberfläche zu **Appearance > Widgets**. Öffnen Sie den Block-Einfüger, suchen Sie nach "FastComments" und wählen Sie **FastComments: Top Pages**. Ziehen Sie es in eine Seitenleiste, den Header oder Footer-Bereich und legen Sie den Titel im Widget-Bedienfeld fest.

### In einem Block-Theme (Full Site Editing)

Öffnen Sie den **Site Editor** unter **Appearance > Editor**. Navigieren Sie zum Template-Teil, in dem das Widget erscheinen soll, fügen Sie einen **Legacy Widget**-Block ein und wählen Sie aus dem Dropdown **FastComments: Top Pages**.

## Fehlerbehebung

Das Widget wird erst angezeigt, nachdem die FastComments-Einrichtung abgeschlossen ist und eine tenant ID gespeichert wurde. Wenn der Widget-Bereich leer ist, schließen Sie die Einrichtung unter **FastComments** in der WordPress-Admin-Oberfläche ab und laden Sie die Seite neu.