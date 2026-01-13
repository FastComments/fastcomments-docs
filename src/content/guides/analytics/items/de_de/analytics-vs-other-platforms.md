Sie werden möglicherweise feststellen, dass unsere Analytics-Metriken leicht unterschiedliche Zahlen zeigen als beispielsweise Google Ads © oder ähnliche Produkte.

Für Websites mit einem Kommentar-Widget pro Seite sind die von FastComments Analytics bereitgestellten Zahlen sehr genau, und wenn sie falsch sind, werden sie **niedriger** als der tatsächliche Wert sein, aber nicht höher.

Wenn Sie eine SPA haben, werden Sie möglicherweise feststellen, dass die FastComments Analytics-Zahlen höher sind als die von Ihren Marketing-Produkten gemeldeten. Dies liegt daran, dass das Marketing-Produkt möglicherweise nur verfolgt, wenn die Seite nicht geladen wird, aber nicht jedes Mal, wenn ein Benutzer etwas auf der Seite tut, das dazu führen könnte, dass ein neuer Kommentar-Thread angezeigt wird, was als Seitenaufruf für FastComments Analytics zählen würde.

#### Technische Informationen

FastComments Analytics verfolgt jeden Seitenaufruf und verlässt sich nicht auf Zufälligkeit als Optimierung. Jeder Seitenaufruf führt dazu, dass ein In-Memory-Zähler in jedem Thread auf jedem Server aktualisiert wird, der dann periodisch über eine atomare Operation in der Datenbank persistiert wird.
