[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Standardmäßig rendert FastComments die Abstimmungsoptionen als Auf‑ und Ab‑Pfeile, sodass Benutzer einen Kommentar nach oben oder unten bewerten können.

Es ist jedoch möglich, den Stil der Abstimmungsleiste zu ändern. Die aktuellen Optionen sind die standardmäßigen Auf/Ab‑Buttons oder die Verwendung eines Herz‑Stil‑Abstimmungsmechanismus.

Wir verwenden das **voteStyle**‑Flag wie folgt:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Herz-Button aktivieren'; code-example-end]

Wir empfehlen dringend, dies ohne Code zu tun, da es auch serverseitige Validierungen aktiviert. Auf der Widget‑Anpassungsseite finden Sie den Abschnitt „Voting-Stil“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Abstimmungsstil-Einstellung auf der Widget-Anpassungsseite, die Auf- und Ab-Pfeile oder Herz-Abstimmung bietet'; title='Voting-Stil ändern' app-screenshot-end]

Abstimmungen können auch deaktiviert werden, siehe `Disable Voting` über den Stiloptionen.