An **AI-agent** er en autonom arbejder, begrænset til din FastComments‑lejer, som overvåger begivenheder i dit fællesskab og handler på dine vegne.

Hver agent har tre ting, du kan kontrollere:

1. **En personlighed.** En fri‑tekst startprompt, der definerer tone, rolle og beslutningsstil ("Du er en varm fællesskabs‑velkomstperson", "Du håndhæver fællesskabsreglerne, men foretrækker advarsel frem for udelukkelse", osv.).
2. **En eller flere udløsere.** En liste over begivenheder, der vækker agenten – en ny kommentar, en kommentar der krydser en stemme‑ eller flag‑tærskel, en moderatorhandling, en brugers første kommentar på siden, og andre. Den fulde liste findes i [Trigger Events Overview](#triggers-overview).
3. **En tilladelsesliste over værktøjer.** Hvad agenten har lov til at gøre – poste en kommentar, stemme, fastgøre, låse, markere som spam, udelukke en bruger, advare via DM, tildele et mærke, sende e‑mail, gemme og søge i delt hukommelse. Den fulde liste findes i [Allowed Tool Calls Overview](#tools-overview).

Når en udløser aktiveres, modtager agenten en kontekstmeddelelse, der beskriver hvad der skete (kommentaren, siden, valgfri tråd/brugerkontekst/sidens kontekst) og får sin startprompt samt dine fællesskabsretningslinjer. Den kalder derefter værktøjer for at handle, og registrerer en begrundelse og en tillidsgrad ved hvert kald.

### Agents run asynchronously

Agenter **blokerer aldrig den brugerhandling, der udløste dem**. En læser posterer en kommentar, kommentaren gemmes og vises i tråden, svaret returneres, og først *derefter* kører agenten på den – enten med det samme eller efter en konfigureret forsinkelse (se [Deferred Triggers](#trigger-deferred-delay)). Intet, hvad agenten gør, tilføjer latenstid til brugeroplevelsen.

### Why use them

- **Moderer i stor skala.** Marker åbenlys spam og udeluk gentagne overtrædere uden at skulle holde øje med køen døgnet rundt.
- **Velkomst til nye kommentatorer.** Svar på førstegangs‑kommentatorer i din stemme.
- **Fremhæv det bedste indhold.** Fastgør væsentlige top‑niveau kommentarer, når de krydser en stemmetærskel.
- **Håndhæv dine retningslinjer konsekvent.** Anvend den samme politiktekst på hver grænse‑kommentar.
- **Opsummér lange tråde.** Post neutrale opsummeringer af debat på tværs af flere sider.

### What keeps you in control

- **Dry‑run‑tilstand.** Hver ny agent leveres i **Dry Run**: den behandler udløsere, kører modellen og registrerer hvad den *ville* gøre, men udfører ingen reelle handlinger. Se [Dry-Run Mode](#dry-run-mode).
- **Godkendelser.** Enhver delmængde af handlinger kan låses bag menneskelig godkendelse. Se [Approval Workflow](#approval-workflow).
- **Per‑agent‑ og per‑konto‑budgetter.** Strenge daglige og månedlige grænser. Se [Budgets Overview](#budgets-overview).
- **Værktøjstilladelsesliste.** Ikke‑tilladte værktøjer fjernes fra modellens palette – agenten kan bogstaveligt talt ikke anmode om dem. Se [Allowed Tool Calls Overview](#tools-overview).
- **Audit‑felter på hver handling.** Modellen skal indeholde en begrundelse og en tillidsgrad. Begge vises i kørselstidslinjen og på hver godkendelse. Se [Run Detail View](#run-detail-view).
- **EU DSA artikel 17.** I EU‑regionen blokeres fuldautomatiske udelukkelser. Se [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Ingen træning på dine data.** FastComments bruger udbydere, der ikke træner på dine prompts eller kommentarer.

### Where they fit alongside human moderation

Agenter og menneskelige moderatorer deler den samme kommentarpplatform: agenter udfører handlinger gennem de samme kanaler (godkend, spam, udeluk, mærke, fastgør, lås, skriv), og disse handlinger vises i de samme [Comment Logs](/guide-moderation.html#comment-logs), den samme [Moderation Page](/guide-moderation.html#moderate-comments-page) og de samme notifikationsstrømme. Agenter og mennesker ser hinandens arbejde og kan hver især reagere på den anden – moderatorhandlinger er i sig selv gyldige agent‑udløsere (se [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) og lignende).

Hvis du også har brug for mennesker i køen, tilbyder FastComments [Human Moderation Services](/guide-moderation.html#human-moderation-services): vores moderatorer arbejder sammen med dine agenter i dit dashboard under dine retningslinjer.

---