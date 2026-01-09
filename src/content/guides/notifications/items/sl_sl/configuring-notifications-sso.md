---
Za SSO obstaja naslednja konfiguracija, ki jo je treba upoštevati za obvestila:

- Ali se je uporabnik odločil za prejemanje obvestil.
  - To se stori z nastavitvijo zastavice `optedInNotifications` na `true` ali `false` v objektu `SSOUser`.
  - To je mogoče nastaviti prek API-ja.
  - Prav tako, če v payload posredujete vrednost za to zastavico, bo ta samodejno posodobljena, ko uporabnik naloži nit komentarjev.
- Ali se je uporabnik odločil za prejemanje obvestil o **naročninah**.
  - To se stori z nastavitvijo zastavice `optedInSubscriptionNotifications` na `true` ali `false` v objektu `SSOUser`.
  - To je mogoče nastaviti prek API-ja.
  - Prav tako, če v payload posredujete vrednost za to zastavico, bo ta samodejno posodobljena, ko uporabnik naloži nit komentarjev.
- Določitev njihovega e-poštnega naslova.
  - Če ni naveden, ne moremo pošiljati obvestil po elektronski pošti.
- Ali onemogočiti povezave za odjavo v e-pošti.
  - To se stori z zastavico `disableUnsubscribeLinks` v objektu `Tenant`.
  - To je mogoče nastaviti prek API-ja.
- Ali uporabiti prilagojeno povezavo za odjavo.
  - To se stori preko lastnosti `footerUnsubscribeURL` na objektu `DomainConfig`.
  - To je mogoče nastaviti prek API-ja.
  - Morda boste želeli tudi razmisliti o nastavitvi ustreznih glavic za odjavo prek `emailHeaders` v istem objektu.

---