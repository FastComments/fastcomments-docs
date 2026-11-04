En **AI-agent** er en autonom arbejder, afgrænset til din FastComments-tenant, som holder øje med hændelser i dit community og handler på dine vegne.

Hver agent har tre ting, du styrer:

1. **En personlighed.** En fritekst-basisprompt, der definerer tone, rolle og beslutningsstil ("Du er en varm community-hilsner", "Du håndhæver community-reglerne, men tenderer mod advarsler frem for udelukkelse" og så videre).
2. **En eller flere triggere.** En liste over hændelser, der vækker agenten - en ny kommentar, en kommentar der krydser en stemme- eller flaggrænse, en moderatorhandling, en brugers første kommentar på sitet og andre. Den fulde liste findes i [Trigger Events Overview](#triggers-overview).
3. **En allowlist af værktøjer.** Hvad agenten må gøre - sende en kommentar, stemme, fastgøre, låse, markere spam, udelukke en bruger, advare via DM, tildele et badge, sende e-mail, gemme og søge i en delt hukommelse. Den fulde liste findes i [Allowed Tool Calls Overview](#tools-overview).

Når en trigger udløses, modtager agenten en kontekstbesked, der beskriver, hvad der skete (kommentaren, siden, valgfri tråd-/bruger-/sidekontekst), og den promptes med sin basisprompt og dine community-retningslinjer. Den kalder derefter værktøjer for at handle og registrerer en begrundelse og en konfidensscore ved hvert kald.

### Agents kører asynkront

Agents **blokerer aldrig den brugerhandling, der udløste dem**. En læser sender en kommentar, kommentaren gemmes og vises i tråden, svaret returneres, og først *derefter* kører agenten på den - enten med det samme eller efter en konfigureret forsinkelse (se [Deferred Triggers](#trigger-deferred-delay)). Intet, agenten gør, tilføjer latens til brugeroplevelsen.

### Hvorfor bruge dem

- **Moderér i skala.** Markér åbenlys spam og udeluk gentagende lovovertrædere uden at overvåge køen døgnet rundt.
- **Byd nye kommentatorer velkommen.** Svar til førstegangs-kommentatorer i din stemme.
- **Fremhæv det bedste indhold.** Fastgør substantielle topniveau-kommentarer, når de krydser en stemmegrænse.
- **Håndhæv dine retningslinjer konsekvent.** Anvend den samme politiktekst på hver grænsetilfælde-kommentar.
- **Opsummér lange tråde.** Send neutrale opsummeringer af flersidede debatter.

### Hvad holder dig i kontrol

- **Dry-run-tilstand.** Enhver ny agent starter i **Dry Run**: den behandler triggere, kører modellen og registrerer, hvad den *ville* gøre, men udfører ingen rigtige handlinger. Se [Dry-Run Mode](#dry-run-mode).
- **Godkendelser.** Enhver delmængde af handlinger kan kræve menneskelig godkendelse. Se [Approval Workflow](#approval-workflow).
- **Budgetter pr. agent og pr. konto.** Hårde daglige og månedlige grænser. Se [Budgets Overview](#budgets-overview).
- **Værktøjs-allowlist.** Ikke-tilladte værktøjer fjernes fra modellens palet - agenten kan bogstaveligt talt ikke anmode om dem. Se [Allowed Tool Calls Overview](#tools-overview).
- **Revisionsfelter på hver handling.** Modellen skal inkludere en begrundelse og en konfidensscore. Begge vises i kørselstidslinjen og ved hver godkendelse. Se [Run Detail View](#run-detail-view).
- **EU DSA Artikel 17.** I EU-regionen er fuldautomatiske udelukkelser blokeret. Se [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Ingen træning på dine data.** FastComments bruger udbydere, der ikke træner på dine prompts eller kommentarer.

### Hvor de passer ind ved siden af menneskelig moderering

Agents og menneskelige moderatorer deler samme kommentarplatform: agents udfører handlinger gennem de samme kanaler (godkend, spam, udeluk, badge, fastgør, lås, skriv), og disse handlinger vises i de samme [Comment Logs](/guide-moderation.html#comment-logs), den samme [Moderation Page](/guide-moderation.html#moderate-comments-page) og de samme notifikationsstrømme. Agents og mennesker ser hinandens arbejde og kan hver reagere på den anden - moderatorhandlinger er selv gyldige agent-triggere (se [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) og venner).

Hvis du også har brug for mennesker i køen, tilbyder FastComments [Human Moderation Services](/guide-moderation.html#human-moderation-services): vores moderatorer arbejder side om side med dine agents, i dit dashboard, under dine retningslinjer.