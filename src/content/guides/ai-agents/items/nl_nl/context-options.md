De **Context**-sectie in het bewerkingsformulier regelt hoeveel informatie de agent bij elke run ontvangt. Meer context leidt tot betere beslissingen maar verhoogt de tokenkosten per run, dus je wilt alleen wat de agent daadwerkelijk nodig heeft.

### Wat altijd wordt opgenomen

Zelfs wanneer alle selectievakjes uitgevinkt zijn, bevat het contextbericht van de agent:

- Het **trigger event type** (bijv. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- De **pagina-URL en URL-ID** (indien bekend).
- De **reactie** die de run heeft getriggerd, als die er is - ID, auteur user ID, weergegeven naam van de auteur, commentaartekst, stem-aantallen, flag-aantal, spam/goedgekeurd/beoordeeld-vlaggen, parent ID. Het e-mailadres van de auteur wordt **nooit** naar de LLM-provider gestuurd (minimalisatie van PII).
- De **vorige commentaartekst** voor `COMMENT_EDIT` triggers (zodat de agent vooraf/achteraf kan vergelijken).
- De **stemrichting** voor `COMMENT_VOTE_THRESHOLD` triggers.
- De **triggerende user ID** en **badge ID** (voor moderator badge triggers).

Alle onbetrouwbare tekst - commentaarteksten, auteursnamen, paginatitels, het richtlijndocument zelf - wordt **afgebakend** in het contextbericht met markers zoals `<<<COMMENT_TEXT>>> ... <<<END>>>`. De systeemprompt van het platform instrueert het model om nooit instructies binnen die afbakening op te volgen. Dit is de prompt-injectieverdediging van het platform; je hoeft dit niet in je prompt te herhalen.

### De drie selectievakjes

#### Inclusief oudercommentaar en eerdere reacties in dezelfde draad

Voegt toe:
- Het **bovenliggende commentaar** - ID, auteur, tekst.
- **Sibling replies** - de eerdere reacties op dezelfde ouder in dezelfde draad.

Nuttig voor: elke agent die op een commentaar reageert in context (welkomstgroeters, samenvatters van discussiedraden, moderators die replies in gesprekken lezen).

Kosten: klein tot medium. Begrensd door hoeveel siblings er in een gegeven draad bestaan.

#### Inclusief vertrouwensfactor van de commenter, accountleeftijd, verbanningsgeschiedenis en recente reacties

Voegt het **AUTHOR_HISTORY**-blok toe:

- **Accountleeftijd in dagen** sinds aanmelding.
- **Trust factor (0-100)** - de FastComments-score die samenvat hoe vertrouwd de gebruiker op deze site is. Zie de [Spamdetectie](/guide-moderation.html#spam-detection)-pagina in de moderatiehandleiding.
- **Aantal eerdere verbanningen.**
- **Totaal aantal reacties op deze site.**
- **Aantal duplicaatberichten** - als de gebruiker recent identieke tekst heeft geplaatst (anti-spam signaal).
- **Zelfde-IP cross-account signaal** - aantal reacties vanaf hetzelfde IP onder andere accounts (alt-account signaal). De IP-hash zelf wordt nooit naar de LLM gestuurd.
- **Recente reacties** - maximaal 5 van de meest recente reacties van de gebruiker, elk afgekapt op 300 tekens, afgebakend als onbetrouwbare tekst.

Nuttig voor: elke moderatie-agent. Zonder dit bant het model nieuwe accounts en lang bestaande goede-gebruikers met dezelfde houding.

Kosten: medium. Recente reacties voegen de meeste tokens toe.

#### Inclusief paginatitel, subtitel, beschrijving en meta-tags

Voegt het **PAGE_CONTEXT**-blok toe - titel, subtitel, beschrijving en eventuele meta-tags die FastComments voor de pagina heeft vastgelegd.

Nuttig voor: welkomstgroeters en samenvatters van discussiedraden, waarbij weten waar de pagina over gaat de kwaliteit van de output aanzienlijk verbetert.

Kosten: klein.

### Communityrichtlijnen

Het vierde veld, **Communityrichtlijnen**, is een vrije-tekst beleidsblok dat bij elke run in het contextbericht van de gebruikersrol wordt opgenomen, afgebakend als onbetrouwbare tekst op dezelfde manier als commentaarteksten en andere door gebruikers aangeleverde inhoud. De agent leest het als beleidstekst, maar het platform behandelt het niet als een systeeminstructie. Zie [Communityrichtlijnen](#community-guidelines) voor wat je erin moet zetten.

### Context selectief toevoegen

Deze selectievakjes gelden per agent, niet globaal. Een veelgebruikt patroon:

- Welkomstgroeter: page context **aan**, thread context **uit**, user history **uit**.
- Moderator: thread context **uit**, user history **aan**, page context **uit**.
- Draad-samenvatter: thread context **aan**, page context **aan**, user history **uit**.

Streef naar de minimale context die een agent nodig heeft om correct te zijn voor de calls die hij daadwerkelijk uitvoert - extra context kost tokens bij elke run, zelfs wanneer de agent deze niet gebruikt.