[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Standaard, als een gebruiker een reactie typt en vervolgens de pagina vernieuwt, het tabblad sluit of weg navigeert voordat hij deze indient, gaat de conceptversie stilletjes verloren.

Het instellen van **warnOnUnsavedComment** op true zorgt ervoor dat de browser de gebruiker vraagt te bevestigen voordat de pagina wordt verlaten terwijl een reactieveld, of een bewerking in uitvoering, nog tekst bevat. Zodra de reactie is ingediend, wordt de tekst gewist, zodat er geen prompt wordt getoond.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Waarschuwing bij niet-opgeslagen commentaar'; code-example-end]

De prompt maakt gebruik van de eigen dialoog van de browser. Moderne browsers tonen hun eigen bewoordingen en negeren aangepaste tekst, waardoor het bericht niet kan worden aangepast.

Deze optie laadt een kleine extensie op aanvraag, waardoor er niets wordt toegevoegd aan de widget voor sites die deze niet inschakelen.