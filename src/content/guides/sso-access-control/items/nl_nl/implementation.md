#### Het noemen van gebruikers in andere groepen

Als twee gebruikers tot twee verschillende sets groepen behoren en er geen overlap is, kunnen ze elkaar niet `@mention`en.

Als een gebruiker handmatig een `@mention` typt en zijn commentaar indient, blijft dit als platte tekst. De andere gebruiker wordt niet getagd.

#### Het beheren van de groepen

`Groups` worden gedefinieerd met behulp van de API-resources `Pages` en `SSOUsers`, respectievelijk.

De `Pages` API kan worden aangeroepen om de set groepen te definiëren die toegang tot de pagina hebben. Standaard hebben alle groepen, en gebruikers die tot geen groep behoren, toegang.

Op dezelfde manier kan de `SSOUsers` API worden aangeroepen om de groepen te definiëren die aan elke gebruiker zijn gekoppeld.

Voor beide resources zijn er geen beperkingen met betrekking tot wanneer de groepen kunnen worden ingesteld of bijgewerkt.

Als het alleen de bedoeling is te voorkomen dat gebruikers elkaar `@mention`'en, hoeven `Pages` niet in overweging te worden genomen.

##### Opmerking!

Het definiëren en bijwerken van de SSO-gebruikersgroepen vereist niet het gebruik van de API en kan in plaats daarvan automatisch worden bijgewerkt door de groep-id's in de SSO-payload op te nemen die naar de commentaar-widget wordt gestuurd. Voor grote lijsten met groepen wordt dit echter niet aanbevolen, omdat de gebruiker deze payload bij elke paginalading zou moeten indienen.