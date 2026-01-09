---
Za SSO treba razmotriti sledeće konfiguracije za obaveštenja:

- Da li je korisnik pristao na obaveštenja.
  - Ovo se postiže podešavanjem `optedInNotifications` na `true` ili `false` u objektu `SSOUser`.
  - Ovo se može podesiti putem API-ja.
  - Takođe, ako prosledite vrednost za ovaj flag u payload-u, ona će se automatski ažurirati kada korisnik učita nit komentara.
- Da li je korisnik pristao na obaveštenja o **pretplati**.
  - Ovo se postiže podešavanjem `optedInSubscriptionNotifications` na `true` ili `false` u objektu `SSOUser`.
  - Ovo se može podesiti putem API-ja.
  - Takođe, ako prosledite vrednost za ovaj flag u payload-u, ona će se automatski ažurirati kada korisnik učita nit komentara.
- Definisanje njihove e-mail adrese.
  - Ako nije prisutna, ne možemo slati obaveštenja putem e-maila.
- Da li onemogućiti linkove za odjavu u e-mailovima.
  - Ovo se radi podešavanjem `disableUnsubscribeLinks` u objektu `Tenant`.
  - Ovo se može podesiti putem API-ja.
- Da li koristiti prilagođeni link za odjavu.
  - Ovo se radi putem svojstva `footerUnsubscribeURL` na objektu `DomainConfig`.
  - Ovo se može podesiti putem API-ja.
  - Takođe možete razmotriti postavljanje relevantnih zaglavlja za odjavu putem `emailHeaders` u istom objektu.

---