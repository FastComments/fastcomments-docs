[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Standaard staat FastComments de gebruiker toe om een commentaar in te voeren met zoveel regels als ze willen, tot aan de standaard tekenlimiet.

Het kan echter wenselijk zijn om de gebruiker te beperken tot het invoeren van slechts één regel tekst. Enkele voorbeeldtoepassingen zijn online bieden of live chat, waarvoor FastComments kan worden gebruikt.

We schakelen de **useSingleLineCommentInput**-vlag in als volgt:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Invoeren van één regel commentaar inschakelen'; code-example-end]

Dit kan ook zonder code worden gedaan. Op de widget-aanpassingspagina, zie de sectie "Invoeren van één regel commentaar inschakelen".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Selectievakje voor invoer van één regel commentaar ingeschakeld op de widget-aanpassingspagina, waardoor invoer wordt beperkt tot één regel'; title='Invoeren van één regel commentaar inschakelen' app-screenshot-end]

Houd er rekening mee dat de commentaren op elke pagina voor elke sorteerrichting vooraf worden berekend, zodat alle sorteerrichtingen dezelfde prestaties hebben.