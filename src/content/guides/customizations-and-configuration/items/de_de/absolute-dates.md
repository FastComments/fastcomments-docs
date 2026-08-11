[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Standardmäßig werden lokalisierte relative Datumsangaben verwendet. Zum Beispiel können Sie neben einem kürzlich abgegebenen Kommentar "vor 11 Minuten" sehen.

Es kann notwendig oder gewünscht sein, absolute Datumsangaben zu verwenden; in diesem Fall setzen Sie diesen Parameter auf true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

Dies kann ohne Code angepasst werden, auf der Widget‑Anpassungsseite unter Erweiterten Optionen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Erweiterte Optionen auf der Widget‑Anpassungsseite mit dem aktivierten Schalter für absolute Datumsangaben'; title='Absolute Datumsangaben verwenden' app-screenshot-end]