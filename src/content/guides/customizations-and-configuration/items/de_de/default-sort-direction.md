[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Standardmäßig sortiert FastComments Kommentare nach der Sortierreihenfolge "Am relevantesten".

Die Sortierung "Am relevantesten" berücksichtigt für die Sortierung den Zeitpunkt, zu dem der Kommentar abgegeben wurde, und die Anzahl der Stimmen.

Der Benutzer kann dann in der Benutzeroberfläche des Kommentar-Widgets die Sortierreihenfolge entweder auf Älteste oder Neueste zuerst ändern.

Wir können das Standardverhalten jedoch auf eine der drei Optionen ändern. Wenn Sie beispielsweise die ältesten Kommentare zuerst anzeigen möchten:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Wir setzen den Wert von **defaultSortDirection** auf "OF", um die Sortierreihenfolge auf "OF" festzulegen.

Für die Sortierreihenfolge Neueste zuerst würden wir Folgendes tun:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Gültige Werte für **defaultSortDirection** sind:

- MR: "Am neuesten"
- NF: "Neueste zuerst"
- OF: "Älteste zuerst"

Dies kann auch ohne Code durchgeführt werden. Auf der Seite zur Anpassung des Widgets siehe den Abschnitt "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Beachten Sie, dass die Kommentare auf jeder Seite für jede Sortierreihenfolge vorab berechnet werden, sodass alle Sortierreihenfolgen die gleiche Leistung haben.