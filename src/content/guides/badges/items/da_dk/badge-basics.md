FastComments User Badges konfigureres af administratorer med `Customize Data`-tilladelsen.

Dette gøres via [Customize -> Badges](https://fastcomments.com/auth/my-account/configure-badges) i dit admin-dashboard.

Når en bruger tildeles et badge, vises det på deres profil og på deres kommentarer.

Når man tilføjer et badge, kan man opsætte en `Display Label`, som er det navn, brugeren ser i forbindelse med badgeet. For eksempel, hvis vi tilføjer et `Comment Count`-badge, vil vi sandsynligvis ikke vise det tekniske navn, da det er ret kedeligt. Vi kunne kalde det `Super Member` eller lignende. Badges kan også stables og erstatte hinanden, hvilket vi kommer ind på senere i dette dokument.

Badges har også konfigurerbare tærskler.

Badges kan oprettes og senere deaktiveres ved at fjerne markeringen i `Enabled`. Deaktivering af et badge betyder, at det ikke længere automatisk tildeles, og det vises ikke i menuen Award Manual Badge, men brugerne beholder badgeet.

### Badge-visningstyper

Badges kan være billeder eller tekst-badges, som understøtter noget grundlæggende styling (tekstfarve, baggrundsfarve og kantfarve). Du kan også style badges via CSS.

Billede-badges kan være GIF-billeder for at vise animerede badges.

### Tip - Fjern ikke badges!

Brugere elsker badges. De plejer virkelig at bryde sig om dem, selv hvis det er en fejl, du tilføjede ved en fejltagelse, og du gerne vil ændre badge-ikonet.

Hvis vi har lært noget, er det, at det er ekstremt svært at tage noget fra brugerne. At fjerne et badge, fordi du som ejer af sitet ikke længere kan lide det, eller ønsker at foretage ændringer, kan resultere i en meget vred skare af brugere, der pludselig forlader dit site i frustration. Af denne grund var `Delete` ikke engang en mulighed i de første par måneder, efter vi udgav denne funktion - men vi endte med at skulle tilføje den. Men brug venligst delete med omtanke. Vi har set mange længerevarende, flerårige brugere blive meget frustrerede og forlade deres fællesskaber, fordi administratorer besluttede at slette et badge.

Hvis du er nødt til at holde op med at bruge et badge, anbefaler vi, at du blot deaktiverer det, så brugerne beholder deres badge.

### Badge-genbehandling

Når et badge tilføjes eller ændres, vil systemet retrospektivt tjekke alle, der har interageret med dit site, for at se, om de bør få badgeet. Dette vil være synligt på Badges-siden i admin-dashboardet, da en spinner vises i stedet for antallet af brugere, der har badgeet. Det skyldes, at antallet af brugere er ved at blive fastlagt.

### Se, hvem der har et badge

I Badges-listen har hvert link en `View Users`-mulighed for at vise listen over brugere, som har optjent eller fået tildelt et badge manuelt.