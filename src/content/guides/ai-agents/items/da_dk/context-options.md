Sektionen **Context** på redigeringsformularen styrer, hvor meget information agenten modtager ved hver kørsel. Mere kontekst giver bedre beslutninger, men øger tokenomkostningerne pr. kørsel, så du kun bør inkludere det, agenten faktisk har brug for.

### Hvad der altid er inkluderet

Selv med alle afkrydsningsfelter fravalgt indeholder agentens kontekstbesked:

- Type for den **udløsende hændelse** (f.eks. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- Siden's **URL og URL-ID** (når kendt).
- Den **kommentar**, der udløste kørslen, hvis der er en - ID, forfatterens bruger-ID, forfatterens visningsnavn, kommentartekst, stemmetællinger, flagtælling, spam/godkendt/gennemgået-flags, parent ID. Forfatterens e-mail sendes **aldrig** til LLM-udbyderen (PII-minimering).
- Den **forrige kommentartekst** for `COMMENT_EDIT`-udløsere (så agenten kan sammenligne før/efter).
- **Stemmeretningen** for `COMMENT_VOTE_THRESHOLD`-udløsere.
- Den **udløsende bruger-ID** og **badge ID** (for moderatorbadge-udløsere).
- Din tenants **badge-katalog** (navn, display label, beskrivelse), når agenten har tilladelse til at tildele badges, så agenten kan vælge en passende uden at du skal angive badges i prompten.

Al ustyret tekst - kommentarkroppe, forfatternavne, sidetitler, selve retningslinjedokumentet - er **indhegnet** i kontekstbeskeden med markører som `<<<COMMENT_TEXT>>> ... <<<END>>>`. Platformens systemprompt instruerer modellen i aldrig at følge instruktioner inde i disse hegn. Dette er platformens forsvar mod prompt-injektion; du behøver ikke gentage det i din prompt.

### De tre afkrydsningsfelter

#### Inkluder forældrekommentaren og tidligere svar i samme tråd

Tilføjer:
- Den **overordnede kommentar** - ID, forfatter, tekst.
- **Søsken-svar** - de tidligere svar til samme forælder i samme tråd.

Brugbart for: enhver agent, der reagerer på en kommentar i kontekst (velkomsthilsener, trådsammenfatninger, moderatorer, der læser svar i samtaler).

Omkostning: lille til medium. Begrænset af, hvor mange søskende der findes i en given tråd.

#### Inkluder kommentatorens tillidsfaktor, kontostatus, forbudshistorik og nylige kommentarer

Tilføjer **AUTHOR_HISTORY**-blokken:

- **Kontens alder i dage** siden tilmelding.
- **Tillidsfaktor (0-100)** - FastComments-score, der opsummerer, hvor betroet brugeren er på dette site. Se [Spam Detection](/guide-moderation.html#spam-detection)-siden i moderationsguiden.
- **Tidligere forbudsantal.**
- **Samlede kommentarer på dette site.**
- **Duplikatindholdstælling** - hvis brugeren for nylig har postet identisk tekst (anti-spam-signal).
- **Samme-IP kryds-konto-signal** - antal kommentarer fra samme IP under andre konti (alt-konto-signal). IP-hashen sendes aldrig til LLM.
- **Nylige kommentarer** - op til 5 af brugerens mest nylige kommentarer, hver trunkeret til 300 tegn, indhegnet som ustyret tekst.

Brugbart for: enhver moderatoragent. Uden dette forbyder modellen nye konti og langtidstro brugere i god tro med samme adfærd.

Omkostning: medium. Nylige kommentarer tilføjer flest tokens.

#### Inkluder sidetitel, undertitel, beskrivelse og meta-tags

Tilføjer **PAGE_CONTEXT**-blokken - titel, undertitel, beskrivelse og eventuelle meta-tags, FastComments har indsamlet for siden.

Brugbart for: velkomsthilsener og trådsammenfatninger, hvor viden om, hvad siden handler om, væsentligt forbedrer outputkvaliteten.

Omkostning: lille.

### Fællesskabsretningslinjer

Det fjerde felt, **Community guidelines**, er en fritekst-politikblok, der inkluderes i bruger-rolle-kontekstbeskeden ved hver kørsel, indhegnet som ustyret tekst på samme måde som kommentarernes kroppe og andet brugergenereret indhold. Agenten læser det som policytekst, men platformen behandler det ikke som en systeminstruktion. Se [Community Guidelines](#community-guidelines) for hvad du skal skrive i det.

### Tilføj kontekst selektivt

Disse afkrydsningsfelter gælder pr. agent, ikke globalt. Et almindeligt mønster:

- Welcome greeter: page context **on**, thread context **off**, user history **off**.
- Moderator: thread context **off**, user history **on**, page context **off**.
- Thread summarizer: thread context **on**, page context **on**, user history **off**.

Stræb efter den minimale mængde kontekst, en agent behøver for at være korrekt i de kald, den faktisk foretager - ekstra kontekst koster tokens ved hver kørsel, selv når agenten ikke bruger den.

---