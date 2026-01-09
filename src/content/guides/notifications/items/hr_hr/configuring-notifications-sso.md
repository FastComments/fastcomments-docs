---
Za SSO postoji sljedeća konfiguracija koju treba razmotriti za obavijesti:

- Je li se korisnik prijavio za primanje obavijesti.
  - To se radi postavljanjem zastavice `optedInNotifications` na `true` ili `false` u objektu `SSOUser`.
  - To se može postaviti putem API-ja.
  - Također, ako u payloadu pošaljete vrijednost za ovu zastavicu, ona će se automatski ažurirati kada korisnik učita nit komentara.
- Je li se korisnik prijavio za obavijesti o pretplati.
  - To se radi postavljanjem zastavice `optedInSubscriptionNotifications` na `true` ili `false` u objektu `SSOUser`.
  - To se može postaviti putem API-ja.
  - Također, ako u payloadu pošaljete vrijednost za ovu zastavicu, ona će se automatski ažurirati kada korisnik učita nit komentara.
- Definiranje njihove e-pošte.
  - Ako nije prisutna, ne možemo slati obavijesti putem e-pošte.
- Hoće li se onesposobiti poveznice za odjavu u e-porukama.
  - To se radi putem zastavice `disableUnsubscribeLinks` u objektu `Tenant`.
  - To se može postaviti putem API-ja.
- Hoće li se koristiti prilagođena poveznica za odjavu.
  - To se radi svojstvom `footerUnsubscribeURL` na objektu `DomainConfig`.
  - To se može postaviti putem API-ja.
  - Možda ćete također htjeti razmotriti postavljanje relevantnih zaglavlja za odjavu putem `emailHeaders` u istom objektu.

---