Das Widget "Neueste Diskussionen" zeigt die Seiten Ihrer Website mit der jüngsten Kommentaraktivität an. Es ist nützlich, um Threads hervorzuheben, zu denen weiterhin Kommentare hinzugefügt werden, damit Besucher sich wieder in aktive Unterhaltungen einklinken können, anstatt auf ruhigen Seiten zu landen.

## Optionen

- **Titel** (optional): Die Überschrift, die über der Liste angezeigt wird. Standardmäßig "Neueste Diskussionen".
- **Anzahl** (optional): Wie viele Diskussionen angezeigt werden sollen. Bereich von 1 bis 50. Standardmäßig 20.

## So fügen Sie es hinzu

### In einem Beitrag oder auf einer Seite

Fügen Sie im Block-Editor einen **Shortcode**-Block hinzu und fügen Sie Folgendes ein:

[inline-code-attrs-start title = 'Shortcode für Neueste Diskussionen'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Das Attribut `count` akzeptiert jeden Wert zwischen 1 und 50.

### In einer Seitenleiste oder Fußzeile (klassische Themes)

Gehen Sie in Ihrem WordPress-Admin zu **Design > Widgets**. Suchen Sie im Block-Einfüger nach "FastComments" und wählen Sie **FastComments: Neueste Diskussionen**. Ziehen Sie es in eine Seitenleiste, Kopfzeile oder Fußzeile und konfigurieren Sie dann Titel und Anzahl im Widget-Bedienfeld.

### In einem Block-Theme (Full Site Editing)

Öffnen Sie den **Site-Editor** unter **Design > Editor**. Navigieren Sie zum Template-Teil, in dem das Widget erscheinen soll, fügen Sie einen **Legacy Widget**-Block ein und wählen Sie **FastComments: Neueste Diskussionen** aus dem Dropdown-Menü.

## Fehlerbehebung

Das Widget wird erst angezeigt, nachdem die FastComments-Einrichtung abgeschlossen ist und eine Tenant-ID gespeichert wurde. Wenn der Widget-Bereich leer ist, schließen Sie die Einrichtung unter **FastComments** im WordPress-Admin ab und laden Sie die Seite neu.

Wenn die Reihenfolge der Diskussionen veraltet aussieht, prüfen Sie, ob die zugrunde liegenden Seiten im FastComments-Dashboard vollständig synchronisiert wurden. Das Widget liest Live-Daten, sodass frisch importierte Kommentare einen Moment dauern können, bis sie angezeigt werden.