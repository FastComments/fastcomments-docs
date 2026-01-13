Met FastComments SSO-toegangscontrole, soms aangeduid als RBAC, kunnen gebruikers beperkt worden tot alleen toegang tot bepaalde pagina's of reactiedraden. Daarnaast,
kunnen gebruikers elkaar alleen `@mention`en binnen dezelfde groep.

## In detail

We kunnen `Users` en optioneel `Pages` in groepen plaatsen.

Wanneer `Users` in groepen worden geplaatst, en `Limit Comments by SSO User Groups` is ingeschakeld in Widget Settings, zullen gebruikers
alleen reacties zien van gebruikers in hun eigen groepen en zullen ze alleen in staat zijn om `@mention` gebruikers in dezelfde groepen.

Daarnaast kunnen `Pages` in groepen worden geplaatst, en dan kunnen gebruikers alleen toegang krijgen tot reacties voor pagina's waarvoor ze toegang hebben.

We noemen dit "User-Level" groepen versus "Page-Level" groepen.

Dus welke is geschikt voor jou?

#### Gebruik User-Level-groepen als...

- Je wilt dezelfde `urlId` waarde (pagina-URL of artikel-ID) gebruiken, maar reacties per groep beperken.
- Bijvoorbeeld, je wilt groepen "New User" en "Veteran User" hebben, en ze mogen elkaars reacties op dezelfde pagina's nooit zien.

#### Gebruik Page-Level-groepen als...

- Je groepen hebben specifieke pagina's.
- Bijvoorbeeld, gebruikers in de groep "Public Pages" mogen nooit artikelen bekijken op de "Top Secret"-pagina's.

---