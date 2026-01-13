SSO, eller single-sign-on, er et sæt konventioner, der bruges til at lade dig eller dine brugere bruge FastComments uden at skulle oprette en separat konto.

Antager vi, at du ikke tillader anonyme kommentarer, kræves der en konto for at kommentere med FastComments. Vi gør denne tilmeldingsproces meget nem - brugeren efterlader bare deres e-mail, når de kommenterer.
Vi forstår dog, at selv det kan være ekstra friktion, som nogle sider ønsker at undgå.

Vi kan reducere den friktion ved kun at have én login-flow for hele dit site.

### How do I get it?
Alle kontotyper får i øjeblikket adgang til SSO. Antallet af SSO-brugere vil dog variere afhængigt af din pakke. Som med andre funktioner tilbyder Pro-planerne og højere direkte udviklingssupport.

Lad os sammenligne mulighederne, og så gå i detaljer om hver.

### User and Comment Migrations

Når du migrerer fra en platform med SSO som Disqus, vil du allerede have brugere og deres kommentarer.

Kommentarer importeres som en del af din migration, enten via API'et, vores Import UI eller kundesupport. Import UI foretrækkes, hvis det understøtter den platform, du
migrerer fra, da det indeholder fejlhåndtering, avatar- og medie-ekstraktion og uploads samt et batch-job-overvågningssystem.

Brugerne tilføjes automatisk, når kommentartråde ses for første gang. Alternativt kan de forudtilføjes via API'et, men dette arbejde har ikke
mange fordele.

Hvis kommentarer er importeret, og SSO-brugere ikke tilføjes manuelt via API'et, vil kommentarerne automatisk blive migreret til brugerens konto første
gang den oprettes, når de ser en hvilken som helst kommentartråd. De vil derefter kunne administrere, redigere og slette de kommentarer, de oprindeligt skrev.

Den automatiske migrering sker via e-mail eller brugernavn. Nogle platforme leverer ikke e-mails ved eksport, som Disqus, så vi falder tilbage til brugernavn i dette tilfælde.
- Så længe du sender et matchende brugernavn og en e-mail i SSO-payloaden, vil vi tilføje e-mailen til de enkelte kommentarobjekter, så notifikationer og mentions fungerer.

Hvis det ønskes at importere dine kommentarer og brugere på én gang, så arbejd med support for at migrere kommentarerne over til brugernes respektive konti efter brugerne er importeret
via API'et.

Sammenfattende er den nemmeste fremgangsmåde til migreringen:

1. Importer kommentarer.
   1. Avatarer og andet medieindhold migreres automatisk, hvis du bruger Import UI under `Manage Data -> Imports`.
2. Opsæt Secure eller Simple SSO.
3. Lad migreringen ske per bruger automatisk, når de logger ind første gang.
   1. Dette tilføjer normalt mindre end et sekund til indlæsningstiden, hvis brugeren har mindre end 50k kommentarer.

### WordPress Users
Hvis du bruger vores <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress-plugin</a> er der ingen kode at skrive! Gå blot til pluginets Admin-side, klik på SSO Settings, og tryk derefter Enable.

Dette fører dig til en enkelt-knap-vejledning, som vil oprette din API-nøgle, sende den til din WordPress-installation og slå SSO til. Vi har konsolideret dette til et enkelt knap-klik for dig.

Bemærk, at hvis du installerer pluginet for første gang, skal du følge opsætningsprocessen, før du ser admin-siden med SSO Settings-knappen.

#### WordPress SSO - Moderators

Bemærk, at for at "Moderator"-badge aktuelt skal vises ved siden af dine moderatorer, når de kommenterer med FastComments WordPress-pluginet,
skal de også være tilføjet som Moderator i FastComments-dashboardet og have deres e-mail bekræftet.

### Custom Integrations

For tilpassede integrationer er der to muligheder.

### Option One - Secure SSO

Med Secure SSO ved FastComments, at brugeren, der kommenterer, stemmer og læser kommentarer, er en reel bruger på dit site.

Så længe du opretter en gyldig payload, vil brugeren altid få en sømløs kommenteringsoplevelse.

Med Secure SSO oprettes SSO-payloaden **på serversiden** ved hjælp af HMAC-autentificering og sendes derefter til widget'en på **klienten**.

Med Secure SSO er brugerens konto **helt adskilt** fra resten af FastComments brugerbase. Det betyder, at hvis vi har to partnere
Company A og Company B, kan hver have en SSO-bruger med brugernavnet "Bob".

#### Requirements
- Nogen grundlæggende viden om backend-udvikling.
- Nogen grundlæggende viden om håndtering af hemmelige API-nøgler.
- Nogen grundlæggende viden om API-udvikling eller server-side rendering.

#### Pros
- Sikkert.
- Sømløs kommenteringsoplevelse.

#### Cons
- Kræver backend-udvikling.

#### Updating User Data

Med Secure SSO vil vi hver gang du sender sso-bruger-payloaden, opdatere deres bruger med de seneste oplysninger. For eksempel, hvis
brugeren har brugernavnet `X`, og du sender `Y` i SSO-payloaden, vil deres brugernavn blive `Y`.

Hvis du vil fjerne værdier ved hjælp af denne tilgang, så sæt dem til `null` (ikke `undefined`).

#### Secure SSO API

Vi tilbyder også et API til at interagere med SSO-brugerne. Se [dokumentationen](/guide-api.html#sso-user-structure).

Bemærk, at når du bruger Secure SSO, oprettes brugere automatisk bag kulisserne ved sideindlæsning. Du behøver ikke at importere dine brugere i bulk.

### Option Two - Simple SSO

Alternativet til Secure SSO er simpelthen at sende brugeroplysningerne til kommenteringswidget'en.

Det er ikke nødvendigt at angive en e-mail med Simple SSO; uden denne vil deres kommentarer dog blive vist som "Uverificeret".

<sup>Bemærk!</sup> Pr. begyndelsen af 2022 behøver brugernavne med Simple SSO ikke at være unikke på tværs af hele FastComments.com.

Ideelt set bør Simple SSO kun vælges, når der udvikles på en platform, der ikke giver adgang til backend.

#### Requirements
- Nogen grundlæggende viden om klient-side udvikling.
- Skal kende mindst brugerens e-mail.

#### Pros
- Simpelt.
- Al aktivitet bliver stadig verificeret.
- Brugeren indtaster aldrig deres brugernavn eller e-mail.

#### Cons
- Mindre sikkert end Secure SSO, da client-side payloaden kunne blive manipuleret til at blive en anden bruger.

#### Simple SSO API

Brugere, der automatisk oprettes via Simple SSO-flowet, gemmes som `SSOUser` objekter. De kan tilgås og administreres via `SSOUser` API'et. Se [dokumentationen](/guide-api.html#sso-user-structure).