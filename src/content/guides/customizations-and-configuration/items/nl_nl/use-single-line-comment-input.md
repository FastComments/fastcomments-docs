[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Standaard staat FastComments de gebruiker toe een reactie in te voeren met zoveel regels als ze willen, tot de standaard tekenlimiet.

Het kan echter wenselijk zijn om de gebruiker te beperken tot het invoeren van slechts één regel tekst. Enkele voorbeeldgebruikssituaties zijn online bieden, of live chat, waarvoor FastComments
kan worden gebruikt.

We schakelen de **useSingleLineCommentInput**-vlag als volgt in:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Dit kan ook zonder code worden gedaan. Op de pagina voor het aanpassen van de widget, zie de sectie "Schakel éénregelige reactie-invoer in".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Let op dat de reacties op elke pagina voor elke sorteerrichting vooraf worden berekend, zodat alle sorteerrichtingen dezelfde prestaties hebben.