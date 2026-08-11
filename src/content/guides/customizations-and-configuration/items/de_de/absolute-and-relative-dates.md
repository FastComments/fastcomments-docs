[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Standardmäßig werden lokalisierte relative Datumsangaben verwendet. Zum Beispiel kann neben einem kürzlich abgegebenen Kommentar "11 minutes ago" stehen.

Es kann notwendig oder gewünscht sein, dieses relative Datumsformat beizubehalten, aber gleichzeitig das vollständige Datum daneben anzuzeigen; in diesem Fall setzen Sie diesen Parameter auf true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Dies kann ohne Code angepasst werden, auf der Widget‑Anpassungsseite unter Erweiterten Optionen. Sie müssen zunächst Absolute Dates aktivieren, um diese Option in der Benutzeroberfläche zu sehen.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='Erweiterte Optionen auf der Widget‑Anpassungsseite mit aktivierten absoluten Datumsangaben und der kombinierten Einstellung für relative und absolute Datumsangaben'; title='Sowohl absolute als auch relative Datumsangaben verwenden' app-screenshot-end]