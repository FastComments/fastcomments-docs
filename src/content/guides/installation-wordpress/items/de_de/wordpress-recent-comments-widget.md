Das "Recent Comments"-Widget zeigt die neuesten Kommentare an, die auf Ihrer gesamten Website veröffentlicht wurden. Es ist nützlich in Sidebars, Footern oder überall dort, wo Sie aktuelle Aktivitäten hervorheben möchten, um zu weiterem Lesen anzuregen.

## Optionen

- **Title** (optional): Die Überschrift, die über der Liste angezeigt wird. Standardmäßig "Recent Comments".
- **Count** (optional): Wie viele Kommentare angezeigt werden sollen. Bereich 1 bis 50. Standardmäßig 5.

## So fügen Sie es hinzu

### Innerhalb eines Beitrags oder einer Seite

Im Block-Editor fügen Sie einen **Shortcode**-Block hinzu und fügen Folgendes ein:

[inline-code-attrs-start title = 'Shortcode für Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Das `count`-Attribut akzeptiert jeden Wert zwischen 1 und 50.

### In einer Sidebar oder im Footer (klassische Themes)

Gehen Sie im WordPress-Admin zu **Design > Widgets**. Öffnen Sie den Block-Einfüger, suchen Sie nach "FastComments" und wählen Sie **FastComments: Recent Comments**. Ziehen Sie es in eine Sidebar-, Header- oder Footer-Fläche und konfigurieren Sie dann Titel und Anzahl im Widget-Bedienfeld.

### In einem Block-Theme (Full Site Editing)

Öffnen Sie den **Site-Editor** unter **Design > Editor**. Navigieren Sie zu dem Template-Teil, in dem das Widget erscheinen soll, fügen Sie einen **Legacy Widget**-Block ein und wählen Sie **FastComments: Recent Comments** aus dem Dropdown-Menü.

## Fehlerbehebung

Das Widget wird erst angezeigt, nachdem die Einrichtung von FastComments abgeschlossen ist und eine tenant ID gespeichert wurde. Wenn der Widget-Bereich leer ist, schließen Sie die Einrichtung unter **FastComments** im WordPress-Admin ab und laden Sie die Seite neu.