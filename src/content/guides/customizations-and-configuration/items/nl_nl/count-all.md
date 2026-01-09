[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Het aantal reacties dat bovenaan de reacties-widget wordt weergegeven kan óf alle "top-level" reacties tonen, dat wil zeggen die reacties die rechtstreeks op de pagina of het artikel zelf reageren, óf het kan een telling zijn van **alle** geneste reacties.

By default, this is `true` - it is a count of the latter - all comments. In oudere versies van de reacties-widget is de
standaardwaarde `false`.

We kunnen het gedrag wijzigen, zodat het een telling is van **alle** geneste reacties door de **countAll** vlag op true te zetten.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Als we willen dat de telling alleen de top-level reacties weerspiegelt, zetten we de vlag op false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Dit kan momenteel niet worden aangepast zonder codewijzigingen.