[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Standardmäßig rendert FastComments die Abstimmungsoptionen als Auf- und Abwärtspfeile, sodass Benutzer einen Kommentar entweder hoch- oder runterstimmen können.

Es ist jedoch möglich, den Stil der Abstimmungsleiste zu ändern. Die aktuellen Optionen sind die standardmäßigen Auf-/Ab-Tasten oder ein Herzstil für die Abstimmung.

Wir verwenden das Flag **voteStyle** wie folgt:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Wir empfehlen dringend, dies ohne Code zu tun, da dadurch auch serverseitige Validierungen aktiviert werden. Auf der Seite zur Widget-Anpassung finden Sie den Abschnitt „Vote Style“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Abstimmungen können auch deaktiviert werden, siehe `Disable Voting` oberhalb der Stiloptionen.

---