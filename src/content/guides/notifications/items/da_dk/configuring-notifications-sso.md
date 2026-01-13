For SSO er følgende konfiguration relevant for notifikationer:

- Om brugeren har valgt at modtage notifikationer.
  - Dette angives ved at sætte flaget `optedInNotifications` til `true` eller `false` i `SSOUser`-objektet.
  - Dette kan sættes via API'en.
  - Desuden, hvis du medsender en værdi for dette flag i payload'en, opdateres det automatisk, når brugeren indlæser en kommentartråd.
- Om brugeren har valgt at modtage **abonnementsnotifikationer**.
  - Dette angives ved at sætte flaget `optedInSubscriptionNotifications` til `true` eller `false` i `SSOUser`-objektet.
  - Dette kan sættes via API'en.
  - Desuden, hvis du medsender en værdi for dette flag i payload'en, opdateres det automatisk, når brugeren indlæser en kommentartråd.
- At definere deres e-mail.
  - Hvis den ikke er til stede, kan vi ikke sende e-mailbaserede notifikationer.
- Om man vil deaktivere afmeldingslinks i e-mails.
  - Dette gøres via flaget `disableUnsubscribeLinks` i `Tenant`-objektet.
  - Dette kan sættes via API'en.
- Om man vil bruge et brugerdefineret afmeldingslink.
  - Dette gøres via egenskaben `footerUnsubscribeURL` på `DomainConfig`-objektet.
  - Dette kan sættes via API'en.
  - Du kan også overveje at sætte de relevante afmeldingsheaders via `emailHeaders` i det samme objekt.