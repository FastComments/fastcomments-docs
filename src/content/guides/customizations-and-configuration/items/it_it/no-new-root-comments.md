[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Impostare `noNewRootComments` su `true` farà sì che il widget nasconda l'area di risposta principale, ma consentirà comunque agli utenti di rispondere
ai commenti figli. Ad esempio, potresti impostarlo in modo condizionale al caricamento della pagina per consentire solo ad alcuni utenti di lasciare commenti di primo livello.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]