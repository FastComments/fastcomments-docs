Za SSO postoji sljedeća konfiguracija koju treba uzeti u obzir za obavještenja:

- Da li je korisnik pristao da prima obavještenja.
  - Ovo se postiže postavljanjem `optedInNotifications` flag-a na `true` ili `false` u `SSOUser` objektu.
  - Ovo se može postaviti preko API-ja.
  - Takođe, ako proslijedite vrijednost za ovaj flag u tijelu zahtjeva, ona će se automatski ažurirati kada korisnik učita thread komentara.
- Da li je korisnik pristao na obavještenja o **pretplati**.
  - Ovo se postiže postavljanjem `optedInSubscriptionNotifications` flag-a na `true` ili `false` u `SSOUser` objektu.
  - Ovo se može postaviti preko API-ja.
  - Takođe, ako proslijedite vrijednost za ovaj flag u tijelu zahtjeva, ona će se automatski ažurirati kada korisnik učita thread komentara.
- Definisanje njihove e-pošte.
  - Ako nije prisutna, ne možemo slati obavještenja putem e-pošte.
- Da li onemogućiti linkove za odjavu u email porukama.
  - Ovo se postiže preko `disableUnsubscribeLinks` flag-a u `Tenant` objektu.
  - Ovo se može postaviti preko API-ja.
- Da li koristiti prilagođeni link za odjavu.
  - Ovo se postiže preko `footerUnsubscribeURL` property-ja na `DomainConfig` objektu.
  - Ovo se može postaviti preko API-ja.
  - Možda ćete takođe htjeti razmotriti postavljanje odgovarajućih zaglavlja za odjavu putem `emailHeaders` u istom objektu.