En agents **værktøjer** er de handlinger, den kan udføre. Agentens redigeringsformular har en sektion **Allowed tool calls**, hvor du sætter flueben ved de værktøjer, denne agent må bruge, og en sektion **Approvals**, hvor du sætter flueben ved de handlinger, der skal kræve menneskelig godkendelse, før de træder i kraft.

Der er tre niveauer for ethvert værktøj:

- **Disallowed** - agenten kan ikke se eller bruge det.
- **Allowed, no approval** - agenten bruger det direkte. Registreres i kørselsloggen.
- **Allowed, with approval** - agentens kald sættes i kø til menneskelig gennemgang og kører kun, når et menneske godkender det.

Disallowed-værktøjer er stille: agenten kan ikke bede om dem, og platformen afviser dem uden yderligere besked. Værktøjer, der kræver godkendelse, går altid gennem [godkendelsesindbakke](#approval-workflow).

### Revisionsspor for hver handling

Hver handling, agenten foretager, bliver registreret med en kort begrundelse (1–2 sætninger, der forklarer hvorfor) og en tillids-score (0.0–1.0). Begge vises i [Visning af kørselsdetaljer](#run-detail-view) og på enhver [godkendelse](#approval-workflow). Søgning i hukommelsen er den ene skrivebeskyttede undtagelse: det bliver ikke registreret som en handling og er altid tilgængeligt uanset allowlist.

### Værktøjsreference

#### Udstationering af kommentarer

Lader agenten poste en kommentar som sig selv. Kommentaren vises offentligt under agentens visningsnavn. Bruges af velkomst- og opsummeringsagenter. Kan omgøres - enhver moderator kan fjerne en dårlig kommentar. Normalt tilladt uden godkendelse; sæt det bag en godkendelse, hvis dit fællesskab kræver, at alle offentlige beskeder gennemgås af mennesker.

#### Redigering af en kommentar

Lader agenten omskrive teksten i en kommentar inden for scope. Den oprindelige tekst bevares i kommentarens revisionslog. Forbehold til snævre tilfælde - slette PII en bruger lækkede, eller rette agentens eget tidligere svar. Ikke til omskrivning af holdninger eller blødgøring af tone. **Overvej kraftigt at sætte dette bag en godkendelse.** Se [Rediger kommentar](#tool-edit-comment) for hele siden.

#### Afstemning på kommentarer

Lader agenten stemme op eller ned på en kommentar. Stemmen tæller mod kommentarens samlede stemmetal som enhver anden stemme. De fleste fællesskaber foretrækker ikke bots, der stemmer; ikke aktiveret i nogen startskabelon. Hvis du tillader det, kan afstemningen omgøres.

#### Fastgør / fjern fastgørelse af en kommentar

Lader agenten fastgøre en kommentar øverst på siden eller fjerne fastgørelsen af en, der allerede er fastgjort. Platformen håndhæver ikke en regel om én fastgørelse per tråd, så en pin-agent bør instrueres i først at fjerne den tidligere fastgjorte kommentar. Bruges af Top Comment Pinner template. Omgøres; normalt tilladt uden godkendelse.

#### Luk / genåbn en kommentar

Lader agenten forhindre yderligere svar under en kommentar eller genåbne muligheden for svar. Den låste kommentar forbliver synlig. Nyttigt til køling af ophedede tråde, ofte kombineret med en udsat genåbning. Omgøres, men synligt for dit fællesskab; overvej at sætte det bag godkendelse i vigtige fællesskaber.

#### Marker / fjern markering af spam

Lader agenten markere en kommentar som spam (skjuler den for læsere og fodrer spam-klassifikatoren) eller fjerne flaget. Det grundlæggende værktøj for enhver moderationsagent. Omgøres. Overvej kraftigt at kræve godkendelse i de første uger, mens du opbygger tillid til agenten.

#### Godkend / fjern godkendelse af en kommentar

Lader agenten vise en tilbageholdt kommentar for læsere eller skjule en allerede synlig. Mest nyttigt for lejere, der tilbageholder nye kommentarer til moderatorgennemgang. Høje indsatser ved at fjerne godkendelsen af en synlig kommentar - overvej at sætte det bag godkendelse.

#### Marker en kommentar som gennemset

Et kø-tilstands-værktøj: markerer en kommentar som "en moderator (eller agent) har kigget på dette." Ændrer ikke synlighed. Lav risiko; sjældent sat bag godkendelse.

#### Tilkendegiv et badge

Lader agenten give en bruger et badge fra din tenants badge-konfiguration. Kan omgøres af en moderator. Sjældent sat bag godkendelse. Agenten skal kende badge-ID'et, så medtag de relevante ID'er i dine [fællesskabets retningslinjer](#community-guidelines) eller dit [startprompt](#personality-prompt).

#### Send e-mail

Lader agenten sende en almindelig tekst-e-mail fra `noreply@fastcomments.com` til en adresse den vælger. Brug sparsommelig - e-mail er det mest omkostningsfulde værktøj, og dårlige e-mails er svære at fortryde. Overvej kraftigt at kræve godkendelse, og send godkendelses-e-mails til den person, der ejer den indbakke, agenten kommer til at sende til.

#### Gem / søg agent-hukommelse

To parrede værktøjer, der læser og skriver til en delt notes-pool om den bruger, en trigger blev affyret for. Hukommelsen deles på tværs af alle agenter i din tenant, så en triage-agents noter informerer en moderatoragents beslutninger. Søgning er skrivebeskyttet og altid tilgængelig; gemning sættes sjældent bag godkendelse. Se [Agent-hukommelsessystem](#agent-memory-system) for det fulde design.

#### Advar en bruger

Sender en privat DM-advarsel til en bruger om en specifik kommentar og registrerer advarslen atomisk i agent-hukommelsen. Platformens eskaleringspolitik er bygget omkring dette værktøj - advar først, udeluk kun hvis brugeren gentager overtrædelsen. Mindre ofte sat bag godkendelse end `ban_user`, men overvej at kræve godkendelse i agentens første uger. Se [Advar bruger](#tool-warn-user) for hele siden.

#### Forbyd en bruger

Det mest konsekvensfulde værktøj, en agent kan kalde. Udelukker en bruger i en fast varighed, eventuelt som en shadow ban, eventuelt også forbyde IP, eventuelt også slette alle brugerens kommentarer. De to destruktive muligheder (IP, delete-all) er sat bag ekstra opt-ins på redigeringsformularen. I EU-regionen kræver alle udelukkelser menneskelig godkendelse (se [EU DSA Artikel 17-overholdelse](#eu-dsa-compliance)). Overvej kraftigt at sætte dette bag godkendelse overalt. Se [Forbyd bruger](#tool-ban-user) for hele siden.

### Undervalg for Ban-værktøjet

Ban-værktøjet eksponerer to destruktive muligheder - delete-all-comments og ban-by-IP - som er skjult for modellen helt, indtil du aktiverer dem via sektionen **Ban options** på redigeringsformularen. Selv hvis modellen hallucinerer parameteren, afviser platformen værdier, du ikke har optet dig ind i. Se [Forbyd bruger](#tool-ban-user).