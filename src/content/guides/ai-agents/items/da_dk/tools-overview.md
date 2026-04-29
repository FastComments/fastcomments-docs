En agents **værktøjer** er de handlinger, den kan udføre. Agentens redigeringsformular har en sektion **Allowed tool calls**, hvor du afkrydser de værktøjer, denne agent må bruge, og en sektion **Approvals**, hvor du afkrydser de handlinger, der skal kræve en menneskelig godkendelse, før de træder i kraft.

Der er tre niveauer for ethvert værktøj:

- **Disallowed** - agenten kan ikke se eller bruge det.
- **Allowed, no approval** - agenten bruger det direkte. Registreres i run history.
- **Allowed, with approval** - agentens kald sættes i kø til menneskelig gennemgang og kører kun, når en person godkender det.

Disallowed-værktøjer er tavse: agenten kan ikke bede om dem, og platformen afviser dem direkte. Værktøjer, der kræver godkendelse, går altid gennem [godkendelsesindbakke](#approval-workflow).

### Audit trail on every action

Hver handling, agenten udfører, registreres med en kort begrundelse (1–2 sætninger, der forklarer hvorfor) og en tillids-score (0.0–1.0). Begge vises i [Visning af kørselsdetaljer](#run-detail-view) og på hver [godkendelse](#approval-workflow). Søgning i hukommelsen er det ene læse-only-undtagelse: det registreres ikke som en handling og er altid tilgængeligt uanset allowlist.

### Tool reference

#### Posting comments

Lader agenten poste en kommentar som sig selv. Kommentaren vises offentligt under agentens visningsnavn. Bruges af greeter- og summarizer-agenter. Reversibel - enhver moderator kan fjerne en dårlig kommentar. Normalt tilladt uden godkendelse; sæt det bag en godkendelse, hvis dit fællesskab kræver, at alle offentligt rettede beskeder skal gennemgås af et menneske.

#### Editing a comment

Lader agenten omskrive teksten i en kommentar, der er indeholdt af omfanget. Den oprindelige tekst bevares i kommentarens revisionslog. Reserver til snævre tilfælde - redigering af PII, som en bruger har lækket, eller rettelse af agentens eget tidligere svar. Ikke til omskrivning af holdninger eller udjævning af tone. **Overvej kraftigt at placere det bag en godkendelse.** Se [Edit comment](#tool-edit-comment) for hele siden.

#### Voting on comments

Lader agenten stemme op eller ned på en kommentar. Stemmen tæller med i kommentarens stemmetal som enhver anden stemme. De fleste fællesskaber foretrækker ikke, at bots stemmer; ikke aktiveret i nogen startskabelon. Hvis du tillader det, er stemmer reversible.

#### Pin / unpin a comment

Lader agenten fastgøre en kommentar til toppen af siden eller fjerne fastgørelsen af en, der allerede er fastgjort. Platformen håndhæver ikke en én-faste pr. tråd-regel, så en pin-agent bør instrueres i først at fjerne den tidligere fastgjorte kommentar. Bruges af Top Comment Pinner template. Reversibel; normalt tilladt uden godkendelse.

#### Lock / unlock a comment

Lader agenten forhindre yderligere svar under en kommentar eller gendanne svar. Den låste kommentar forbliver synlig. Nyttigt til nedkøling af ophedede tråde, kombineret med en forsinket oplåsning. Reversibel men synlig for dit fællesskab; overvej at placere det bag en godkendelse i højrisikofællesskaber.

#### Mark / unmark spam

Lader agenten markere en kommentar som spam (skjuler den for læsere og fodrer spamklassifikatoren) eller fjerne dette flag. Basisværktøjet for enhver moderationsagent. Reversibel. Overvej kraftigt at kræve godkendelse de første uger, mens du opbygger tillid til agenten.

#### Approve / un-approve a comment

Lader agenten vise en tilbageholdt kommentar for læsere eller skjule en allerede synlig. Mest nyttigt på tenants, der tilbageholder nye kommentarer til moderatorgennemgang. Høj risiko ved at af-approve en synlig kommentar - overvej at kræve godkendelse.

#### Mark a comment reviewed

Et kø-tilstandsværktøj: markerer en kommentar som "en moderator (eller agent) har set på dette." Ændrer ikke synligheden. Lav risiko; sjældent påkrævet godkendelse.

#### Award a badge

Lader agenten give en bruger et badge, du har konfigureret for din tenant. Reversibel af en moderator. Sjældent gated. Når dette værktøj er aktiveret, kan agenten se dine tenants badges og vælge det rigtige af sig selv, så du ikke behøver at indsætte badge-identifikatorer i dine fællesskabsretningslinjer eller i den indledende prompt. Hvis du vil styre, hvilket badge der tildeles for hvilken adfærd, referer til badges ved deres **Display Label** i prompten.

#### Send email

Lader agenten sende en almindelig tekst-e-mail til forfatteren af en kommentar inden for triggerens omfang. Agenten ser aldrig modtagerens e-mailadresse - den vælger en kommentar, og platformen leverer til den adresse, den kommentator angav, da de postede. Fra-adressen er din tenants brandede afsender (med DKIM), når kommentarens domæne matcher et konfigureret domæne, ellers platformens standard. Brug sparsomt - e-mail er det mest indgribende værktøj, og dårlige e-mails er svære at fortryde. Overvej kraftigt at placere det bag en godkendelse, og rout godkendelses-e-mails til den person, der ejer den indbakke, agenten vil komme til at sende fra.

#### Save / search agent memory

To sammenkoblede værktøjer, der læser og skriver i en delt note-pool om den bruger, en trigger blev affyret for. Hukommelsen deles på tværs af alle agenter i din tenant, så noterne fra en triage-agent informerer en moderatoragents beslutninger. Search er read-only og altid tilgængelig; saving er sjældent gated. Se [Agent-hukommelsessystem](#agent-memory-system) for det fulde design.

#### Warn a user

Sender en privat DM-advarsel til en bruger om en specifik kommentar og gemmer atomisk advarslen i agenthukommelsen. Platformens eskalationspolitik er bygget omkring dette værktøj - advar først, udeluk kun ved gentagelse. Mindre ofte gated end `ban_user`, men overvej gating i de første uger af en agents levetid. Se [Warn user](#tool-warn-user) for hele siden.

#### Ban a user

Det mest afgørende værktøj, en agent kan kalde. Udelukker en bruger i en fastsat periode, eventuelt som en shadow ban, eventuelt også udelukkelse efter IP, eventuelt også sletning af alle brugerens kommentarer. De to destruktive muligheder (IP, delete-all) er gated bag ekstra opt-ins på redigeringsformularen. I EU-regionen kræver alle udelukkelser menneskelig godkendelse (se [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Overvej kraftigt at placere det bag en godkendelse overalt. Se [Ban user](#tool-ban-user) for hele siden.

### Ban-tool sub-options

Ban-værktøjet eksponerer to destruktive muligheder - delete-all-comments og ban-by-IP - som er skjult for modellen helt, indtil du aktiverer dem via sektionen **Ban options** på redigeringsformularen. Selv hvis modellen hallucinere parameteren, afviser platformen værdier, du ikke har optet ind i. Se [Ban user](#tool-ban-user).

---