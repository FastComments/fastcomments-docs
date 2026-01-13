[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Standardmäßig ist bei FastComments das Live-Kommentieren aktiviert.

Das bedeutet, dass jeder Betrachter des Kommentarthreads denselben Inhalt sehen sollte.

Zum Beispiel: Wenn ein Kommentar hinzugefügt wird, sollte dieser Kommentar angezeigt werden. Wird ein Kommentar bearbeitet oder entfernt, werden diese Änderungen für alle Betrachter des Threads übernommen. Gleiches gilt für Stimmen und alle Moderationsaktionen.

Wir können dies jedoch deaktivieren:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Dies kann auch ohne Code erfolgen. Auf der Seite zur Anpassung des Widgets finden Sie den Abschnitt "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---