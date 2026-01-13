[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Standaard worden antwoorden op opmerkingen op hoofdniveau weergegeven.

Dit kan zo worden geconfigureerd dat de gebruiker op "Toon antwoorden" bij de opmerkingen op hoofdniveau moet klikken om de onderliggende reacties te zien.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Dit kan zonder code worden aangepast op de widget-aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Deze instelling heeft geen invloed op het aantal opmerkingen op hoofdniveau dat aanvankelijk wordt geladen. Als u één opmerking op hoofdniveau en 29 kinderen heeft, ziet u met deze instelling:

- Ziet de opmerking op hoofdniveau.
- Ziet 'Toon antwoorden (29)' onder deze opmerking.

Als u alle opmerkingen op hoofdniveau wilt weergeven in combinatie met deze optie, stel [de startpagina in op -1](#starting-page).

---