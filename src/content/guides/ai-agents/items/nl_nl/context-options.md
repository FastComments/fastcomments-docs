Het gedeelte **Context** op het bewerkingsformulier bepaalt hoeveel informatie de agent bij elke run ontvangt. Meer context leidt tot betere beslissingen, maar verhoogt de tokenkosten per run, dus je wilt alleen wat de agent daadwerkelijk nodig heeft.

### Wat altijd is inbegrepen

Zelfs als alle selectievakjes uitgeschakeld zijn, bevat het contextbericht van de agent:

- Het **type triggerevent** (bijv. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- De **pagina-URL en URL ID** (indien bekend).
- De **commentaar** die de run heeft geactiveerd, indien aanwezig - ID, gebruikers-ID van de auteur, weergavenaam van de auteur, commentaartekst, aantal stemmen, aantal vlaggen, spam/goedgekeurd/beoordeeld-vlaggen, ouder-ID. Het e-mailadres van de auteur wordt **nooit** naar de LLM-provider gestuurd (PII-minimalisatie).
- De **vorige commentaartekst** bij `COMMENT_EDIT`-triggers (zodat de agent voor/na kan vergelijken).
- De **stemrichting** bij `COMMENT_VOTE_THRESHOLD`-triggers.
- De **gebruikers-ID van de triggerende gebruiker** en **badge-ID** (voor moderator-badge-triggers).
- De **badgecatalogus** van je tenant (naam, weergavelabel, beschrijving) wanneer de agent badges mag toekennen, zodat de agent een passende kan kiezen zonder dat je de badges in de prompt hoeft uit te schrijven.

Alle niet-vertrouwde tekst - commentaarinhoud, auteursnamen, paginatitels, het richtlijnendocument zelf - wordt in het contextbericht **omheind** met markeringen zoals `<<<COMMENT_TEXT>>> ... <<<END>>>`. De systeemprompt van het platform instrueert het model om nooit instructies binnen die omheiningen op te volgen. Dit is de prompt-injectieverdediging van het platform; je hoeft deze niet te herhalen in je prompt.

### De drie selectievakjes

#### Oudercommentaar en eerdere reacties in dezelfde thread opnemen

Voegt toe:
- Het **oudercommentaar** - ID, auteur, tekst.
- **Reacties van siblings** - de eerdere reacties op hetzelfde oudercommentaar in dezelfde thread.

Nuttig voor: elke agent die op een commentaar reageert met context (welkomstgroeters, thread-samenvatters, moderators die reacties in gesprekken lezen).

Kosten: klein tot middelgroot. Begrensd door het aantal siblings dat op een bepaalde thread bestaat.

#### Trustfactor, accountleeftijd, banhistorie en recente commentaren van de commentergever opnemen

Voegt het **AUTHOR_HISTORY**-blok toe:

- **Accountleeftijd in dagen** sinds registratie.
- **Trustfactor (0-100)** - de FastComments-score die samenvat hoe vertrouwd de gebruiker op deze site is. Zie de pagina [Spam Detection](/guide-moderation.html#spam-detection) in de moderatiegids.
- **Aantal eerdere bans.**
- **Totaal aantal commentaren op deze site.**
- **Aantal duplicaat-inhoud** - als de gebruiker recentelijk identieke tekst heeft geplaatst (anti-spamsignaal).
- **Signaal voor cross-account op hetzelfde IP** - aantal commentaren vanaf hetzelfde IP onder andere accounts (alt-account-signaal). De IP-hash zelf wordt nooit naar de LLM gestuurd.
- **Recente commentaren** - tot 5 meest recente commentaren van de gebruiker, elk afgekapt tot 300 tekens, omheind als niet-vertrouwde tekst.

Nuttig voor: elke moderatie-agent. Zonder dit bans het model nieuwe accounts en langdurige gebruikers met goede bedoelingen op dezelfde manier.

Kosten: middelgroot. Recente commentaren voegen de meeste tokens toe.

#### Paginatitel, subtitel, beschrijving en meta-tags opnemen

Voegt het **PAGE_CONTEXT**-blok toe - titel, subtitel, beschrijving en alle meta-tags die FastComments voor de pagina heeft vastgelegd.

Nuttig voor: welkomstgroeters en thread-samenvatters, waar het weten waar de pagina over gaat de uitvoerkwaliteit aanzienlijk verbetert.

Kosten: klein.

### Communityrichtlijnen

Het vierde veld, **Communityrichtlijnen**, is een vrij tekstbeleidblok dat bij elke run wordt opgenomen in het contextbericht voor de gebruikersrol, op dezelfde manier omheind als niet-vertrouwde tekst zoals commentaarinhoud en andere door gebruikers geleverde inhoud. De agent leest het als beleidstekst, maar het platform behandelt het niet als systeeminstructie. Zie [Communityrichtlijnen](#community-guidelines) voor wat je erin moet zetten.

### Context selectief toevoegen

Deze selectievakjes gelden per agent, niet globaal. Een veelvoorkomend patroon:

- Welkomstgroeter: paginacontext **aan**, threadcontext **uit**, gebruikershistorie **uit**.
- Moderator: threadcontext **uit**, gebruikershistorie **aan**, paginacontext **uit**.
- Thread-samenvatter: threadcontext **aan**, paginacontext **aan**, gebruikershistorie **uit**.

Gebruik de minimale context die een agent nodig heeft om correct te zijn bij de aanroepen die het daadwerkelijk doet - extra context kost tokens bij elke run, zelfs wanneer de agent deze niet gebruikt.