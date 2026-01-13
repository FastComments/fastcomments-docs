---
Voor SSO is de volgende configuratie relevant voor meldingen:

- Of de gebruiker zich heeft aangemeld voor meldingen.
  - Dit gebeurt door de vlag `optedInNotifications` op `true` of `false` te zetten in het `SSOUser` object.
  - Dit kan via de API worden ingesteld.
  - Als u bovendien een waarde voor deze vlag in de payload meegeeft, wordt deze automatisch bijgewerkt wanneer de gebruiker een reactiedraad laadt.
- Of de gebruiker zich heeft aangemeld voor **abonnement**meldingen.
  - Dit gebeurt door de vlag `optedInSubscriptionNotifications` op `true` of `false` te zetten in het `SSOUser` object.
  - Dit kan via de API worden ingesteld.
  - Als u bovendien een waarde voor deze vlag in de payload meegeeft, wordt deze automatisch bijgewerkt wanneer de gebruiker een reactiedraad laadt.
- Het opgeven van hun e-mailadres.
  - Als dit niet aanwezig is, kunnen we geen e-mailgebaseerde meldingen versturen.
- Of afmeldlinks in e-mails uitgeschakeld moeten worden.
  - Dit gebeurt via de vlag `disableUnsubscribeLinks` in het `Tenant` object.
  - Dit kan via de API worden ingesteld.
- Of er een aangepaste afmeldlink wordt gebruikt.
  - Dit gebeurt via de eigenschap `footerUnsubscribeURL` op het `DomainConfig` object.
  - Dit kan via de API worden ingesteld.
  - U kunt ook overwegen om de relevante afmeldheaders in te stellen via `emailHeaders` in hetzelfde object.

---