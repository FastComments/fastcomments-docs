Afsnittet **Context** på redigeringsformularen styrer, hvor meget information agenten modtager ved hver kørsel. Mere kontekst giver bedre beslutninger, men øger token-omkostningen per kørsel, så du kun bør sende det, agenten faktisk har brug for.

### Hvad der altid er inkluderet

Selv når alle afkrydsningsfelter er fravalgt, inkluderer agentens kontekstbesked:

- Den **udløsende hændelsestype** (fx `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- Sidens **URL og URL-ID** (når kendt).
- Den **kommentar**, der udløste kørselen, hvis der er en - ID, forfatterens bruger-ID, forfatterens visningsnavn, kommentarens tekst, stemmetal, antal flag, spam/approved/reviewed flags, parent ID. Forfatterens e-mail bliver **aldrig** sendt til LLM-udbyderen (PII-minimering).
- Den **tidligere kommentartekst** for `COMMENT_EDIT`-udløsere (så agenten kan sammenligne før/efter).
- **Stemmeretningen** for `COMMENT_VOTE_THRESHOLD`-udløsere.
- Det **udløsende bruger-ID** og **badge ID** (for moderator badge-udløsere).

Alt ikke-pålideligt tekstindhold - kommentarindhold, forfatternavne, sidetitler, selve retningslinjedokumentet - er **indhegnet** i kontekstbeskeden med markører som `<<<COMMENT_TEXT>>> ... <<<END>>>`. Platformens systemprompt instruerer modellen om aldrig at følge instruktioner inden for disse hegn. Dette er platformens forsvar mod prompt-injektion; du behøver ikke gentage det i din prompt.

### De tre afkrydsningsfelter

#### Inkluder forældrekommentaren og tidligere svar i samme tråd

Tilføjer:
- Den **forældrekommentar** - ID, forfatter, tekst.
- **Søskende-svar** - de tidligere svar til samme forælder i samme tråd.

Brugbart til: enhver agent der svarer på en kommentar i kontekst (velkomstagent, trådsummerere, moderatorer der læser svar i samtaler).

Omkostning: lille til medium. Begrænset af hvor mange søskende der findes i en given tråd.

#### Inkluder kommentatorens tillidsfaktor, kontoalder, banhistorik og nylige kommentarer

Tilføjer blokken **AUTHOR_HISTORY**:

- **Kontoalder i dage** siden oprettelse.
- **Tillidsfaktor (0-100)** - FastComments-score, der opsummerer hvor betroet brugeren er på dette site. Se [Spam-detektion](/guide-moderation.html#spam-detection)-siden i moderationguiden.
- **Tidligere antal udelukkelser.**
- **Samlede kommentarer på dette site.**
- **Antal dubleret indhold** - hvis brugeren for nylig har postet identisk tekst (anti-spam-signal).
- **Samme-IP tværs-konto-signal** - antal kommentarer fra den samme IP under andre konti (alt-konto-signal). IP-hashen sendes aldrig til LLM'en.
- **Nylige kommentarer** - op til 5 af brugerens seneste kommentarer, hver afkortet til 300 tegn, indhegnet som ikke-pålidelig tekst.

Brugbart til: enhver moderation-agent. Uden dette udelukker modellen nye konti og langvarige troværdige brugere med samme adfærd.

Omkostning: medium. Nylige kommentarer tilfører flest tokens.

#### Inkluder sidetitel, undertitel, beskrivelse og meta-tags

Tilføjer blokken **PAGE_CONTEXT** - titel, undertitel, beskrivelse og eventuelle meta-tags, som FastComments har indsamlet for siden.

Brugbart til: velkomsthilsener og trådsummerere, hvor det i høj grad forbedrer outputkvaliteten at vide, hvad siden handler om.

Omkostning: lille.

### Fællesskabsretningslinjer

Det fjerde felt, **Community guidelines**, er et fritekst-policyfelt, der inkluderes i bruger-rolle-kontekstbeskeden ved hver kørsel, indhegnet som ikke-pålidelig tekst på samme måde som kommentarindhold og andet brugerleveret indhold. Agenten læser det som policytekst, men platformen behandler det ikke som en systeminstruks. Se [Fællesskabsretningslinjer](#community-guidelines) for hvad du skal skrive i det.

### Tilføj kontekst selektivt

Disse afkrydsningsfelter gælder per agent, ikke globalt. Et almindeligt mønster:

- Velkomstagent: sidekontekst **til**, trådkontekst **fra**, brugerhistorik **fra**.
- Moderator: trådkontekst **fra**, brugerhistorik **til**, sidekontekst **fra**.
- Trådsummerer: trådkontekst **til**, sidekontekst **til**, brugerhistorik **fra**.

Vælg kun den minimale kontekst, en agent har brug for for at være korrekt i de kald, den rent faktisk foretager - ekstra kontekst koster tokens ved hver kørsel, selv når agenten ikke bruger den.