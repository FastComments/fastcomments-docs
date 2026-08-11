[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Standardmäßig ist Live‑Kommentierung in FastComments aktiviert.

Das bedeutet, dass jeder Betrachter des Kommentar‑Threads denselben Inhalt sehen sollte.

Zum Beispiel, wenn ein Kommentar hinzugefügt wird, sollte dieser Kommentar angezeigt werden. Wenn ein Kommentar bearbeitet oder entfernt wird,
werden diese Kommentare für alle Betrachter des Threads bearbeitet oder entfernt. Gleiches gilt für Stimmen und alle Moderationsaktionen.

Wir können dies jedoch deaktivieren:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Live-Kommentierung deaktivieren'; code-example-end]

Dies kann auch ohne Code durchgeführt werden. Auf der Widget‑Anpassungsseite finden Sie den Abschnitt „Live‑Kommentierung deaktivieren“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Abschnitt „Live‑Kommentierung deaktivieren“ der Widget‑Anpassungsseite, der die Echtzeit‑Thread‑Aktualisierungen ausschaltet'; title='Live‑Kommentierung deaktivieren' app-screenshot-end]