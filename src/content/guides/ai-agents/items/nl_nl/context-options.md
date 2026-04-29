De **Context**-sectie op het bewerkingsformulier bepaalt hoeveel informatie de agent bij elke uitvoering ontvangt. Meer context leidt tot betere beslissingen maar verhoogt de tokenkosten per uitvoering, dus u wilt alleen wat de agent daadwerkelijk nodig heeft.

### Wat altijd wordt opgenomen

Zelfs als elk selectievakje is uitgevinkt, bevat het contextbericht van de agent:

- Het **trigger event type** (bijv. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- De **pagina-URL en URL-ID** (indien bekend).
- De **reactie** die de uitvoering heeft geactiveerd, als die er is - ID, auteur gebruikers-ID, auteur weergavenaam, reactietekst, stemtellingen, flag-aantal, spam/goedgekeurd/beoordeeld-vlaggen, parent ID. Het e‑mailadres van de auteur wordt **nooit** naar de LLM-provider gestuurd (PII-minimalisatie).
- De **vorige reactietekst** voor `COMMENT_EDIT` triggers (zodat de agent vóór/na kan vergelijken).
- De **stemrichting** voor `COMMENT_VOTE_THRESHOLD` triggers.
- De **gebruiker-ID** en **badge-ID** die de trigger veroorzaakten (voor moderator-badge triggers).
- De **badgecatalogus** van uw tenant (naam, weergavelabel, beschrijving) wanneer de agent badges mag toekennen, zodat de agent een geschikte kan kiezen zonder dat u de badges in de prompt hoeft te beschrijven.

Alle niet-vertrouwde tekst - reactieteksten, auteursnamen, paginatitels, het richtlijndocument zelf - wordt in het contextbericht **afgeschermd** met markeringen zoals `<<<COMMENT_TEXT>>> ... <<<END>>>`. De systeemprompt van het platform instrueert het model om nooit instructies binnen die afschermingen op te volgen. Dit is de prompt-injectie-afweer van het platform; u hoeft dit niet in uw prompt te herhalen.

### De drie selectievakjes

#### Include parent comment and prior replies in the same thread

Voegt toe:
- De **parent comment** - ID, auteur, tekst.
- **Sibling replies** - de eerdere reacties op dezelfde parent in dezelfde thread.

Handig voor: elke agent die op een reactie in context reageert (welkomsgroeters, thread-samenvatters, moderators die reacties in gesprekken lezen).

Kosten: klein tot middelgroot. Begrensd door hoeveel siblings er in een thread bestaan.

#### Include commenter's trust factor, account age, ban history, and recent comments

Voegt het **AUTHOR_HISTORY**-blok toe:

- **Account age in days** sinds aanmelding.
- **Trust factor (0-100)** - de FastComments-score die samenvat hoe betrouwbaar de gebruiker op deze site is. Zie de [Spam Detection](/guide-moderation.html#spam-detection) pagina in de moderatiehandleiding.
- **Prior ban count.**
- **Total comments on this site.**
- **Duplicate-content count** - als de gebruiker recent identieke tekst heeft gepost (anti-spam signaal).
- **Same-IP cross-account signal** - aantal reacties vanaf hetzelfde IP onder andere accounts (alt-account signaal). De IP-hash zelf wordt nooit naar de LLM gestuurd.
- **Recent comments** - tot 5 van de meest recente reacties van de gebruiker, elk afgekapt op 300 tekens, afgeschermd als niet-vertrouwde tekst.

Handig voor: elke moderatieagent. Zonder dit verbant het model nieuwe accounts en lang bestaande goede gebruikers met dezelfde houding.

Kosten: middelgroot. Recente reacties voegen de meeste tokens toe.

#### Include page title, subtitle, description, and meta tags

Voegt het **PAGE_CONTEXT**-blok toe - titel, subtitel, beschrijving en eventuele meta-tags die FastComments voor de pagina heeft vastgelegd.

Handig voor: welkomsgroeters en thread-samenvatters, waarbij het weten waar de pagina over gaat de outputkwaliteit aanzienlijk verbetert.

Kosten: klein.

### Community guidelines

Het vierde veld, **Community guidelines**, is een vrije-tekst beleidsblok dat bij elke uitvoering in het contextbericht met de rol van gebruiker wordt opgenomen, afgeschermd als niet-vertrouwde tekst op dezelfde manier als reactieteksten en andere door gebruikers aangeleverde inhoud. De agent leest het als beleidsdocument, maar het platform behandelt het niet als een systeeminstructie. Zie [Community Guidelines](#community-guidelines) voor wat u erin moet zetten.

### Context selectief toevoegen

Deze selectievakjes gelden per agent, niet globaal. Een veelvoorkomend patroon:

- Welkomsgroeter: page context **on**, thread context **off**, user history **off**.
- Moderator: thread context **off**, user history **on**, page context **off**.
- Thread-samenvatter: thread context **on**, page context **on**, user history **off**.

Kies de minimale context die een agent nodig heeft om correct te zijn voor de calls die hij daadwerkelijk uitvoert - extra context kost tokens bij elke uitvoering, zelfs wanneer de agent het niet gebruikt.