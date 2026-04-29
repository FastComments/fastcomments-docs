En agents **værktøjer** er de handlinger, den kan udføre. Agent-redigeringsformularen har en sektion **Allowed tool calls**, hvor du markerer de værktøjer, denne agent må bruge, og en sektion **Approvals**, hvor du markerer de handlinger, der skal kræve menneskelig godkendelse, før de træder i kraft.

Der er tre niveauer for ethvert værktøj:

- **Disallowed** - agenten kan ikke se eller bruge det.
- **Allowed, no approval** - agenten bruger det direkte. Registreres i run history.
- **Allowed, with approval** - agentens kald sættes i kø til menneskelig gennemgang og kører kun, når et menneske godkender det.

Disallowed-værktøjer er tavse: agenten kan ikke anmode om dem, og platformen afviser dem direkte. Godkendelsesbegrænsede værktøjer går altid gennem [approvals inbox](#approval-workflow).

### Revisionsspor for hver handling

Hver handling agenten udfører registreres med en kort begrundelse (1–2 sætninger, der forklarer hvorfor) og en tillids-score (0.0–1.0). Begge vises i [Run Detail View](#run-detail-view) og på hver [approval](#approval-workflow). Søgning i hukommelsen er det eneste læse-only-undtagelse: det registreres ikke som en handling og er altid tilgængeligt uanset allowlist.

### Værktøjsreference

#### Poste kommentarer

Giver agenten mulighed for at poste en kommentar som sig selv. Kommentaren vises offentligt under agentens visningsnavn. Bruges af velkomst- og opsummeringsagenter. Kan gøres om — enhver moderator kan fjerne en dårlig kommentar. Normalt tilladt uden godkendelse; sæt det bag godkendelse, hvis dit fællesskab kræver menneskelig gennemgang af alle offentlige beskeder.

#### Stemning på kommentarer

Giver agenten mulighed for at stemme op eller ned på en kommentar. Stemmen tæller med i kommentarens samlede stemmetal som enhver anden stemme. De fleste fællesskaber foretrækker ikke bots, der stemmer; ikke aktiveret i nogen startskabelon. Hvis du tillader det, er stemning kan gøres om.

#### Pin / unpin en kommentar

Giver agenten mulighed for at fastgøre en kommentar til toppen af siden eller fjerne fastgørelsen af en allerede fastgjort kommentar. Platformen håndhæver ikke en-flag-per-tråd-regel, så en fastgørelsesagent bør instrueres i først at fjerne den tidligere fastgjorte kommentar. Bruges af Top Comment Pinner-skabelonen. Kan gøres om; normalt tilladt uden godkendelse.

#### Lås / oplås en kommentar

Giver agenten mulighed for at forhindre yderligere svar under en kommentar eller genåbne muligheden for svar. Den låste kommentar forbliver synlig. Nyttigt for afkøling af ophedede tråde, ofte parret med en forsinket oplåsning. Kan gøres om, men synligt for dit fællesskab; overvej at placere det bag godkendelse i højrisiko-fællesskaber.

#### Marker / afmarker spam

Giver agenten mulighed for at markere en kommentar som spam (skjuler den for læsere og fodrer spam-klassificeren) eller fjerne det flag. Det grundlæggende værktøj for enhver moderatoragent. Kan gøres om. Overvej kraftigt at sætte det bag godkendelse de første uger, mens du opbygger tillid til agenten.

#### Godkend / af-godkend en kommentar

Giver agenten mulighed for at vise en tilbageholdt kommentar for læsere eller skjule en allerede synlig kommentar. Mest brugbart på tenants, der tilbageholder nye kommentarer til moderatorgennemgang. Høj indsats ved af-godkendelse af en synlig kommentar - overvej at kræve godkendelse.

#### Marker en kommentar som gennemset

Et kø-tilstands-værktøj: markerer en kommentar som "en moderator (eller agent) har set dette." Ændrer ikke synlighed. Lav risiko; sjældent begrænset.

#### Tildel et badge

Giver agenten mulighed for at give en bruger et badge fra din tenants badge-konfiguration. Kan gøres om af en moderator. Sjældent begrænset. Agenten skal kende badge-ID'et, så medtag de relevante ID'er i dine [community guidelines](#community-guidelines) eller [initial prompt](#personality-prompt).

#### Send e-mail

Giver agenten mulighed for at sende en plain-text e-mail fra `noreply@fastcomments.com` til en adresse, den vælger. Brug sparsomt - e-mail er det værktøj med højest friktion, og dårlige e-mails er svære at fortryde. Overvej kraftigt at sætte det bag godkendelse, og route godkendelses-e-mails til den, der ejer den indbakke, agenten ender med at sende til.

#### Gem / søg agent-hukommelse

To parrede værktøjer, der læser og skriver i en delt note-pool om den bruger, en trigger blev udløst for. Hukommelsen deles på tværs af alle agenter i din tenant, så en triage-agents noter informerer en moderatoragents beslutninger. Søgning er læse-only og altid tilgængelig; gemning er sjældent begrænset. Se [Agenthukommelsessystem](#agent-memory-system) for det fulde design.

#### Advar en bruger

Sender en privat DM-advarsel til en bruger om en bestemt kommentar og registrerer advarslen atomisk i agent-hukommelsen. Platformens eskaleringspolitik er bygget omkring dette værktøj - advar først, udeluk kun, hvis brugeren gentager overtrædelsen. Mindre ofte begrænset end `ban_user`, men overvej at sætte det bag godkendelse i agentens første uger. Se [Warn user](#tool-warn-user) for hele siden.

#### Udeluk en bruger

Det mest vidtrækkende værktøj, en agent kan kalde. Udelukker en bruger med en fastsat varighed, valgfrit som en shadow-ban, valgfrit også ved at udelukke IP, valgfrit også slette alle brugerens kommentarer. De to destruktive valgmuligheder (IP, slet-alle) er placeret bag ekstra opt-ins på redigeringsformularen. I EU-regionen kræver alle udelukkelser menneskelig godkendelse (se [EU DSA artikel 17-overholdelse](#eu-dsa-compliance)). Overvej kraftigt at placere det bag godkendelse overalt. Se [Udeluk bruger](#tool-ban-user) for hele siden.

### Udeluk-værktøjets underindstillinger

Udeluk-værktøjet eksponerer to destruktive muligheder - delete-all-comments og ban-by-IP - som er skjult for modellen helt, indtil du aktivt vælger dem via sektionen **Ban options** på redigeringsformularen. Selv hvis modellen hallucinere parameteren, afviser platformen værdier, du ikke har aktiveret. Se [Udeluk bruger](#tool-ban-user).