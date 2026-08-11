[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

Standardmäßig befindet sich der Kommentar‑Eingabebereich **vor** dem Kommentar‑Thread. Durch Setzen dieses Konfigurationsparameters
auf true können wir ihn jedoch **nach** verschieben.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

Dies kann ohne Code auf der Widget‑Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; alt='Widget-Anpassungsseite-Option, die das Eingabefeld für Kommentare nach dem Kommentar-Thread anstatt davor platziert'; title='Verschieben des Antwortfeldes nach unten' app-screenshot-end]