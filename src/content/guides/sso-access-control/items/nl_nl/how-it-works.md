FastComments Access Control werkt door `Pages` en `Users` aan de gewenste groepen toe te wijzen.

Een groep is simpelweg een string-identificator, zoals `GREEN` of `abc-123`.

`Users` en `Pages` zijn niet beperkt tot slechts één groep. Ze zijn respectievelijk beperkt tot `100` en `1000` groepen. 

#### Toegang tot niet-geautoriseerde pagina's

Als een gebruiker probeert toegang te krijgen tot een pagina waarvoor hij/zij geen toegang heeft, ziet hij/zij een foutmelding zoals hieronder:

<div class="screenshot white-bg">
    <div class="title">Voorbeeld van autorisatiefout</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Voorbeeld van autorisatiefout" />
</div>

De berichttekst kan worden aangepast.