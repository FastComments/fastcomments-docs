---
[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

Standardmäßig werden lokalisierte relative Datumsangaben verwendet. Zum Beispiel sehen Sie neben einem kürzlich abgegebenen Kommentar möglicherweise "vor 11 Minuten".

Es kann notwendig oder wünschenswert sein, dieses relative Datumsformat beizubehalten und zugleich das vollständige Datum daneben anzuzeigen; in diesem Fall setzen Sie diesen Parameter auf true. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

Dies kann ohne Code auf der Seite zur Anpassung des Widgets unter Advanced Options angepasst werden. Sie müssen zunächst Absolute Dates aktivieren, um diese Option in der UI zu sehen.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---