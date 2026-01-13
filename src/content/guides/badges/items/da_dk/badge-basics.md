FastComments-brugerbadges konfigureres af administratorer med tilladelsen `Customize Data`.

Dette gøres via [Tilpas -> Badges](https://fastcomments.com/auth/my-account/configure-badges) i dit administrationspanel.

Når en bruger tildeles et badge, vises det på deres profil og på deres kommentarer.

Når du tilføjer et badge kan vi opsætte en `Display Label`, som er det navn brugeren ser i forbindelse med badget. For eksempel, hvis vi tilføjer et `Comment Count` badge
vil vi sandsynligvis ikke vise det tekniske navn, fordi det er ret kedeligt. Vi kunne kalde det `Super Member` eller lignende. Badges kan også stables og erstatte hinanden, som vi vil gennemgå
senere i dette dokument.

Badges har også konfigurerbare tærskler.

Badges kan oprettes og senere deaktiveres ved at fjerne markeringen i `Enabled`. At deaktivere et badge betyder, at det ikke længere tildeles automatisk, og det vil ikke vises i menuen 'Tildel badge manuelt', men
brugerne beholder badget.

### Badge Display Types

Badges kan være billeder eller tekst-badges, som understøtter nogle grundlæggende stilarter (tekstfarve, baggrundsfarve og kantfarve). Du kan også style badges via CSS.

Billed-badges kan være GIF-billeder for at vise animerede badges.

### Tip - Do Not Remove Badges!

Brugere elsker badges. De går ofte meget op i dem, selv hvis det er en fejl, du tilføjede ved et uheld, og du ønsker at ændre badge-ikonet.

Hvis vi har lært noget, er det ekstremt svært at tage noget fra brugere. At fjerne et badge fordi du som ejer af siden ikke længere kan lide det, eller ønsker at lave ændringer, kan resultere i en meget vred gruppe brugere, som pludselig forlader din side i frustration. Af denne grund var `Delete` ikke engang en mulighed i de første par måneder efter vi frigav denne funktion - dog endte vi med at skulle tilføje den. Men vær venlig, brug slet med forsigtighed. Vi har
set mange langtidsbrugere, der har været med i flere år, blive meget frustrerede og forlade deres fællesskaber, fordi administratorer besluttede at slette et badge.

Hvis du må stoppe med at bruge et badge, foreslår vi, at du blot deaktiverer det, så brugerne beholder deres badge.

### Badge Reprocessing

Når et badge tilføjes eller ændres, vil systemet bagudrettet tjekke alle, der har interageret med din side, for at se om de skal have badget. Dette vil være
synligt på Badges-siden i administrationspanelet, da et spinnerikon vil blive vist i stedet for antallet af brugere, der har badget. Dette er fordi antallet af brugere
er ved at blive fastlagt.

### Seeing Who Has a Badge

På Badges-listen har hvert link en `View Users`-mulighed for at vise listen over brugere, som har fortjent eller manuelt tildelt et badge.