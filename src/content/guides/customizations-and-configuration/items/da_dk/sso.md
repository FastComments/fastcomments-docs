SSO, eller single-sign-on, er et sæt konventioner, der giver dig eller dine brugere mulighed for at bruge FastComments uden at skulle oprette endnu en konto.

Hvis du ikke tillader anonym kommentering, kræves der en konto for at kommentere med FastComments. Vi gør denne tilmeldingsproces meget nem - brugeren skal blot angive deres e-mail, når de kommenterer.
Vi forstår dog, at selv det er ekstra friktion, som nogle sider ønsker at undgå.

Vi kan reducere den friktion ved kun at have ét login-flow for hele din side.

### Hvordan får jeg det?
Alle kontotyper har i øjeblikket adgang til SSO. Det maksimale antal SSO-brugere varierer dog afhængigt af din pakke. Som med andre funktioner giver Pro-planer og derover direkte udviklersupport.

Lad os sammenligne mulighederne og derefter gå i detaljer med hver af dem.

### Bruger- og kommentarmigrering

Når du migrerer fra en platform med SSO som Disqus, har du allerede brugere og deres kommentarer.

Kommentarer importeres som en del af din migrering, enten via API'en, vores Import UI eller kundesupport. Import UI foretrækkes, hvis den understøtter den platform, du
migrerer fra, da den indeholder fejlhåndtering, udtrækning og upload af avatarer og medier samt et batchjob-overvågningssystem.

Brugerne selv tilføjes automatisk, første gang de ser en kommentartråd. Alternativt kan de forudtilføjes via API'en, men dette arbejde har ikke
mange fordele.

Hvis kommentarer importeres, og SSO-brugere ikke tilføjes manuelt via API'en, vil kommentarerne automatisk blive migreret til brugerens konto, første gang
den oprettes, når de ser en kommentartråd. De vil derefter kunne administrere, redigere og slette de kommentarer, de oprindeligt skrev.

Den automatiske migrering sker via e-mail eller brugernavn. Nogle platforme giver ikke e-mails ved eksport, som Disqus, så vi falder tilbage på brugernavn i dette tilfælde.
- Så længe du angiver et matchende brugernavn og en e-mail i SSO-payload'en, tilføjer vi e-mailen til de enkelte kommentarobjekter, så notifikationer og omtaler fungerer.

Hvis du ønsker at importere dine kommentarer og brugere på én gang, så samarbejd med support om at migrere kommentarerne over til brugernes respektive konti, efter brugerne er importeret
via API'en.

For at opsummere den nemmeste vej for migreringen:

1. Importér kommentarer.
   1. Avatarer og andre medier migreres automatisk, hvis du bruger Import UI under `Manage Data -> Imports`.
2. Opsæt Secure eller Simple SSO.
3. Lad migreringen ske pr. bruger automatisk, når de logger ind for første gang.
   1. Dette tilføjer normalt mindre end et sekund til sidens indlæsningstid, hvis brugeren har færre end 50k kommentarer.

### WordPress-brugere
Hvis du bruger vores <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress-plugin</a>, er der ingen kode at skrive! Gå blot til pluginets Admin-side, klik på SSO Settings og derefter Enable.

Dette fører dig til en guide med én enkelt knap, som opretter din API-nøgle, sender den til din WordPress-installation og slår SSO til. Vi har konsolideret dette til ét enkelt knapklik for dig.

Bemærk, at hvis du installerer pluginet for første gang, skal du gennemføre opsætningsprocessen, før du ser admin-siden med SSO Settings-knappen.

#### WordPress SSO - Moderatorer

Bemærk, at i øjeblikket, for at "Moderator"-mærket skal vises ved siden af dine moderatorer, når de kommenterer med FastComments WordPress-pluginet,
skal de også tilføjes som Moderator i FastComments-dashboardet og have deres e-mail verificeret.

### Brugerdefinerede integrationer

For brugerdefinerede integrationer er der to muligheder.

### Mulighed ét - Secure SSO

Med Secure SSO ved FastComments, at brugeren, der kommenterer, stemmer og læser kommentarer, er en rigtig bruger på din side.

Så længe du opretter en gyldig payload, vil brugeren altid have en problemfri kommentaroplevelse.

Med Secure SSO oprettes SSO-payload'en **server-side** ved hjælp af HMAC-godkendelse og sendes derefter til widgetten på **klientsiden**.

Med Secure SSO er brugerens konto **helt adskilt** fra resten af FastComments' brugerbase. Det betyder, at hvis vi har to partnere,
Company A og Company B, kan hver især have en SSO-bruger med brugernavnet "Bob".

#### Krav
- Grundlæggende viden om backend-udvikling.
- Grundlæggende viden om håndtering af hemmelige API-nøgler.
- Grundlæggende viden om API-udvikling eller server-side rendering.

#### Fordele
- Sikkert.
- Problemfri kommentaroplevelse.

#### Ulemper
- Kræver backend-udvikling.

#### Opdatering af brugerdata

Med Secure SSO opdaterer vi brugerens konto med de nyeste oplysninger, hver gang du sender SSO-bruger-payload'en. For eksempel, hvis
brugeren har brugernavnet `X`, og du sender `Y` i SSO-payload'en, bliver deres brugernavn `Y`.

Hvis du vil fjerne værdier med denne tilgang, så sæt dem til `null` (ikke `undefined`).

#### Secure SSO API

Vi tilbyder også en API til at interagere med SSO-brugerne. Se [dokumentationen](/guide-api.html#sso-user-structure).

Bemærk, at når du bruger Secure SSO, oprettes brugerne automatisk i baggrunden ved sideindlæsning. Du behøver ikke bulk-importere dine brugere.

### Mulighed to - Simple SSO

Alternativet til Secure SSO er blot at sende brugeroplysningerne til kommentar-widgetten.

Det er ikke påkrævet at angive en e-mail med Simple SSO, men uden denne vises deres kommentarer som "Unverified".

<sup>Bemærk!</sup> Fra begyndelsen af 2022 behøver brugernavne med Simple SSO ikke længere at være unikke på tværs af hele FastComments.com.

Ideelt set bør Simple SSO kun vælges, når man udvikler på en platform, der ikke giver backend-adgang.

#### Krav
- Grundlæggende viden om klientudvikling.
- Man skal kende mindst brugerens e-mail.

#### Fordele
- Simpelt.
- Al aktivitet verificeres stadig.
- Brugeren indtaster aldrig deres brugernavn eller e-mail.

#### Ulemper
- Mindre sikkert end Secure SSO, da klient-side-payload'en kunne manipuleres til at blive enhver bruger.

#### Simple SSO API

Brugere, der automatisk oprettes via Simple SSO-flowet, gemmes som `SSOUser`-objekter. De kan tilgås og administreres via `SSOUser`-API'en. Se [dokumentationen](/guide-api.html#sso-user-structure).