---
[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Standardmäßig werden lokalisierte relative Datumsangaben verwendet. Zum Beispiel sehen Sie neben einem kürzlich abgegebenen Kommentar möglicherweise "vor 11 Minuten".

Es kann erforderlich oder gewünscht sein, absolute Datumsangaben zu verwenden; in diesem Fall setzen Sie diesen Parameter auf true. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Absolute Datumsangaben verwenden'; code-example-end]

Dies kann ohne Programmieraufwand auf der Seite zur Anpassung des Widgets unter Erweiterten Optionen konfiguriert werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Absolute Datumsangaben verwenden' app-screenshot-end]

---