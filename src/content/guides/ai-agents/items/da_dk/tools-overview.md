En agents **værktøjer** er de handlinger, den kan udføre. Agentens redigeringsformular har en sektion **Tilladte værktøjskald** hvor du markerer de værktøjer, denne agent må bruge, og en sektion **Godkendelser** hvor du markerer de handlinger, der skal kræve menneskelig godkendelse, før de træder i kraft.

Der er tre niveauer for ethvert værktøj:

- **Ikke tilladt** - agenten kan ikke se eller bruge det.
- **Tilladt, ingen godkendelse** - agenten bruger det direkte. Registreres i kørselshistorikken.
- **Tilladt, med godkendelse** - agentens kald sættes i kø til menneskelig gennemgang og kører kun, når en person godkender.

Ikke-tilladte værktøjer er stille: agenten kan ikke anmode om dem, og platformen afviser dem uden videre. Værktøjer, der kræver godkendelse, går altid gennem [approvals inbox](#approval-workflow).

### Revisionsspor for hver handling

Hver handling, agenten udfører, registreres med en kort begrundelse (1–2 sætninger, der forklarer hvorfor) og en tillids-score (0.0–1.0). Begge vises i [Run Detail View](#run-detail-view) og ved hver [approval](#approval-workflow). Søgning i hukommelsen er det eneste skrivebeskyttede undtagelse: det registreres ikke som en handling og er altid tilgængeligt uanset allowlisten.

### Værktøjsreference

#### Poste kommentarer

Giver agenten mulighed for at poste en kommentar som sig selv. Kommentaren vises offentligt under agentens visningsnavn. Bruges af velkomst- og opsummeringsagenter. Kan gøres om - enhver moderator kan fjerne en dårlig kommentar. Stil den bag en godkendelse, hvis dit fællesskab kræver, at alle offentlige beskeder gennemgås af et menneske.

#### Redigering af en kommentar

Giver agenten mulighed for at omskrive teksten i en kommentar, der er inden for omfanget. Den oprindelige tekst bevares i kommentarens revisionslog. Forbeholdes snævre tilfælde - slette PII en bruger har lækket, eller rette agentens egen tidligere svar. Ikke til omskrivning af holdninger eller blødgøring af tone. Se [Edit comment](#tool-edit-comment) for hele siden.

#### Stemmer på kommentarer

Giver agenten mulighed for at stemme op eller ned på en kommentar. Stemmen tæller mod kommentarens stemmetal som enhver anden stemme. De fleste fællesskaber foretrækker, at bots ikke stemmer; ikke aktiveret i nogen starter-skabelon. Hvis du tillader det, er stemmegivning reversibel.

#### Fastgør / fjern fastgørelse af en kommentar

Giver agenten mulighed for at fastgøre en kommentar øverst på siden eller fjerne fastgørelsen af en kommentar, der allerede er fastgjort. Platformen håndhæver ikke en én-fastgørelse-pr-tråd-regel, så en fastgørelsesagent bør instrueres i først at fjerne den tidligere fastgjorte kommentar. For at finde ud af, hvad der allerede er fastgjort på samme side, kan agenten kalde det skrivebeskyttede `get_pinned_comments` værktøj (se nedenfor). Bruges af Top Comment Pinner-skabelonen.

#### Lås / oplås en kommentar

Giver agenten mulighed for at forhindre yderligere svar under en kommentar, eller gendanne svarmuligheden. Den låste kommentar forbliver synlig. Nyttigt til afkøling af ophedede tråde, ofte parret med en forsinket oplåsning. For at finde ud af, hvad der i øjeblikket er låst på samme side, kan agenten kalde det skrivebeskyttede `get_locked_comments` værktøj (se nedenfor).

#### Marker / ophæv markering som spam

Giver agenten mulighed for at markere en kommentar som spam (skjuler den for læsere og fodrer spamklassificatoren) eller fjerne det flag. Grundlæggende værktøj for enhver moderationsagent. Reversibelt.

#### Godkend / fjern godkendelse af en kommentar

Giver agenten mulighed for at vise en holdt kommentar for læsere, eller skjule en allerede synlig kommentar. Mest nyttigt på lejere, der holder nye kommentarer til moderatorgennemgang.

#### Marker en kommentar som gennemset

Et køstatus-værktøj: markerer en kommentar som "en moderator (eller agent) har set på denne." Ændrer ikke synlighed. Lav risiko; sjældent bag godkendelse.

#### Tildel et badge

Giver agenten mulighed for at give en bruger et badge, du har konfigureret for din tenant. Reversibelt af en moderator. Når dette værktøj er aktiveret, kan agenten se din tenants badges og selv vælge det rigtige, så du ikke behøver at indsætte badge-identifikatorer i dine fællesskabsretningslinjer eller i den indledende prompt. For at styre, hvilket badge der tildeles for hvilken adfærd, henvis til badges ved deres **Visningsetiket** i prompten.

#### Send e-mail

Giver agenten mulighed for at sende en almindelig tekst-e-mail til forfatteren af en kommentar inden for triggerens omfang. Agenten ser aldrig modtagerens e-mailadresse - den vælger en kommentar, og platformen leverer til den adresse, som kommentatoren opgav ved opslaget. Afsenderadressen er din tenants brandede afsender (med DKIM), når kommentarens domæne matcher et konfigureret domæne; ellers platformens standard. Brug sparsomt - e-mail er det mest friktionfyldte værktøj, og dårlige e-mails er svære at rette.

#### Gem / søg i agenthukommelse

To sammenkoblede værktøjer, der skriver og læser fra en delt note-pulje om den bruger, en trigger blev udløst for. Hukommelsen deles på tværs af alle agenter i din tenant, så en triage-agents noter informerer en moderatoragents beslutninger. Søgning er skrivebeskyttet og altid tilgængelig; gemning er sjældent bag godkendelse. Se [Agenthukommelsessystem](#agent-memory-system) for det fulde design.

#### Hent fastgjorte kommentarer / Hent låste kommentarer

To skrivebeskyttede opdagelsesværktøjer, der lister de fastgjorte (eller låste) kommentarer på den samme side (`urlId`), som triggeren blev udløst på. De tager ingen argumenter - siden læses fra triggerkonteksten, så agenten kan ikke skifte til andre sider. Brug dem, når en agent skal handle på en kommentar, der allerede er fastgjort eller låst - typisk det første kald før `unpin_comment` eller `unlock_comment`, eller før fastgørelse af en ny kommentar, så den eksisterende først kan fjernes.

Hvert værktøj er gated separat i **Tilladte værktøjskald** (administratoren markerer `List pinned comments on the current page` eller `List locked comments on the current page`). De kan ikke sættes bag godkendelse - skrivebeskyttede værktøjer har ingen bivirkning, der kan godkendes. At kalde dem registreres ikke som en handling i kørselshistorikken; kun det resulterende `unpin_comment` / `unlock_comment` / `pin_comment` kald (hvis nogen) vises. Listen er begrænset til de seneste 20 matches pr. kald.

Vigtigt at forstå: når et af disse værktøjer returnerer et commentId, bliver det commentId tilføjet til agentens per-kørsel-omfang, så det efterfølgende `unpin_comment` / `unlock_comment` kald validerer imod platformens værktøj-mål-sikkerhedstjek. Uden først at have kaldt opdagelsesværktøjet kan agenten ikke handle på kommentarer, der ikke allerede er i triggerens umiddelbare omfang. Så en unpin-agtig agent får typisk begge værktøjer aktiveret (f.eks. `get_pinned_comments` plus `unpin_comment`).

#### Advar en bruger

Sender en privat DM-advarsel til en bruger om en specifik kommentar og registrerer atomisk advarslen i agenthukommelsen. Platformens eskalationspolitik er bygget omkring dette værktøj - advar først, ban kun hvis brugeren gentager forseelsen. Se [Warn user](#tool-warn-user) for hele siden.

#### Ban en bruger

Det mest afgørende værktøj, en agent kan kalde. Baner en bruger i en fastsat periode, valgfrit som en shadow ban, valgfrit også med bannlyst IP, valgfrit også sletning af alle brugerens kommentarer. De to destruktive muligheder (IP, slet-alle) er gated bag ekstra opt-ins i redigeringsformularen. I EU-regionen kræver alle bans menneskelig godkendelse (se [EU DSA Article 17 Compliance](#eu-dsa-compliance)). Se [Ban user](#tool-ban-user) for hele siden.

### Ban-værktøjets underindstillinger

Ban-værktøjet eksponerer to destruktive muligheder - delete-all-comments og ban-by-IP - som er skjult for modellen helt indtil du aktiverer dem via sektionen **Ban options** i redigeringsformularen. Selv hvis modellen hallucinere parameteren, afviser platformen værdier, du ikke har aktiveret. Se [Ban user](#tool-ban-user).