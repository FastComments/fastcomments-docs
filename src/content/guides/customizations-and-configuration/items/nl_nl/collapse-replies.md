[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Standaard worden antwoorden op top-level reacties weergegeven.

Dit kan zo worden geconfigureerd dat de gebruiker op "Show Replies" moet klikken op de top-level reacties om de antwoorden te zien.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Antwoorden op top-level reacties inklappen'; code-example-end]

Dit kan zonder code worden aangepast op de widget-aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='Optie om antwoorden in te klappen in de widget-aanpassingsinterface, waarbij kindreacties verborgen worden achter een Show Replies‑link'; title='Antwoorden inklappen' app-screenshot-end]

Deze instelling heeft geen invloed op het aantal top-level reacties dat aanvankelijk wordt geladen. Als je één top-level reactie hebt, en 29 antwoorden, zul je met deze instelling:

- De top-level reactie zien.
- De "Show Replies" (29) onder deze reactie zien.

Als je alle top-level reacties wilt weergeven in combinatie met deze optie, stel dan [startpagina op -1](#starting-page) in.