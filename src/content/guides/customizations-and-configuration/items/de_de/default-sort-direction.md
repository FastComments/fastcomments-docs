[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Standardmäßig sortiert FastComments Kommentare nach der Sortierrichtung „Relevanteste“.

Die Sortierung nach „Relevanteste“ berücksichtigt sowohl die Zeit, zu der der Kommentar abgegeben wurde, als auch die Anzahl der Stimmen.

Der Benutzer kann dann die Sortierrichtung im UI des Kommentar‑Widgets entweder zu „Älteste zuerst“ oder zu „Neueste zuerst“ ändern.

Wir können die Standardeinstellung jedoch auf eine der drei Optionen ändern. Zum Beispiel, wenn Sie die ältesten Kommentare zuerst anzeigen möchten:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Ändern der Standardsortierung zu Älteste zuerst'; code-example-end]

Wir setzen den Wert von **defaultSortDirection** auf „OF“, um die Richtung auf „OF“ zu setzen.

Für die Sortierrichtung „Neueste zuerst“ würden wir Folgendes tun:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Ändern der Standardsortierung zu Neueste zuerst'; code-example-end]

Die gültigen Werte für **defaultSortDirection** sind:

- MR: "Neueste"
- NF: "Neueste zuerst"
- OF: "Älteste zuerst"

Dies kann auch ohne Code durchgeführt werden. Auf der Seite zur Widget‑Anpassung finden Sie den Abschnitt „Standard‑Sortierrichtung“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Standard Sortier‑Richtung‑Selektor, der Relevanteste, Neueste zuerst und Älteste zuerst anbietet'; title='Ändern der Standard‑Sortier‑Richtung' app-screenshot-end]

Beachten Sie, dass die Kommentare auf jeder Seite für jede Sortierrichtung vorab berechnet werden, sodass alle Sortierrichtungen die gleiche Leistung haben.