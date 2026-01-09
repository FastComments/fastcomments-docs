---
[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Standardmäßig erkennt das FastComments-Kommentar-Widget auf den meisten Websites automatisch den Dunkelmodus.

Wenn der Dunkelmodus erkannt wird, wechselt FastComments von schwarzem Text auf weißem Hintergrund zu weißem Text auf schwarzem Hintergrund. Auch Bilder werden angepasst.

Beim Laden der Seite versucht das Widget zu ermitteln, wie dunkel der Hintergrund der Seite hinter dem Kommentar-Widget ist. Das bedeutet, die Seite könnte einen weißen Hintergrund haben, aber wenn Sie das Kommentar-Widget in einen Container mit schwarzem Hintergrund setzen, sollte der Dunkelmodus trotzdem automatisch aktiviert werden, damit die Kommentare lesbar sind.

Der Erkennungsmechanismus, der auf der Bestimmung der "luminance" beruht, kann den Dunkelmodus jedoch möglicherweise nicht dann aktivieren, wenn Sie es wünschen. Um ihn zwangsweise zu aktivieren, setzen Sie das Flag *hasDarkBackground* auf true wie folgt:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---