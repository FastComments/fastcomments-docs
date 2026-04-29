En **AI-agent** er en autonom arbejder, scoped til dit FastComments-tenant, der overvåger begivenheder i dit fællesskab og handler på dine vegne.

Hver agent har tre ting, du kontrollerer:

1. **En personlighed.** En frit-tekstet indledende prompt, der definerer tone, rolle og beslutningsstil ("Du er en varm fællesskabsvelkommer", "Du håndhæver fællesskabsreglerne, men vælger advarsel frem for udelukkelse", og så videre).
2. **En eller flere udløsere.** En liste over begivenheder, der vækker agenten - en ny kommentar, en kommentar der krydser en stemme- eller flaggrænse, en moderatorhandling, en brugers første kommentar på sitet og andre. Den fulde liste findes i [Oversigt over udløserbegivenheder](#triggers-overview).
3. **En tilladelsesliste over værktøjer.** Hvad agenten må gøre - indsende en kommentar, stemme, fastgøre, låse, markere som spam, udelukke en bruger, advare via DM, tildele et mærke, sende e-mail, gemme og søge i et delt memory. Den fulde liste findes i [Oversigt over tilladte værktøjskald](#tools-overview).

Når en udløser aktiveres, modtager agenten en kontekstbesked, der beskriver hvad der skete (kommentaren, siden, valgfri tråd-/bruger-/sidekontekst) og får vist sit indledende prompt samt dine fællesskabsretningslinjer. Den kalder derefter værktøjer for at handle og registrerer en begrundelse og en tillids-score for hvert kald.

### Agenter kører asynkront

Agenter blokerer **aldrig den brugerhandling, der udløste dem**. En læser poster en kommentar, kommentaren gemmes og vises i tråden, svaret returneres, og først *derefter* kører agenten på den — enten med det samme eller efter en konfigureret forsinkelse (se [Udskudte udløsere](#trigger-deferred-delay)). Intet af det, agenten gør, tilføjer latens til den brugerrettede oplevelse.

### Hvorfor bruge dem

- **Moderér i stor skala.** Markér åbenlys spam og udeluk gentagne overtrædere uden at overvåge køen døgnet rundt.
- **Velkomne nye kommentatorer.** Svar første-gangs kommentatorer med din stemme.
- **Fremhæv det bedste indhold.** Fastgør substansfulde topniveau-kommentarer, når de passerer en stemmegrænse.
- **Håndhæv dine retningslinjer konsekvent.** Anvend den samme politiktekst på hver tvivlsomme kommentar.
- **Opsummer lange tråde.** Post neutrale sammenfatninger af debatter på flere sider.

### Hvad holder dig i kontrol

- **Tørkørselstilstand.** Hver ny agent leveres i **Tørkørsel**: den behandler udløsere, kører modellen og registrerer hvad den *ville* gøre, men foretager ingen reelle handlinger. Se [Tørkørselstilstand](#dry-run-mode).
- **Godkendelser.** Enhver delmængde af handlinger kan kræve menneskelig godkendelse. Se [Godkendelsesarbejdsgang](#approval-workflow).
- **Budgetter pr. agent og pr. konto.** Hårde daglige og månedlige loft. Se [Budgetoversigt](#budgets-overview).
- **Værktøjs-tilladelsesliste.** Ikke-tilladte værktøjer fjernes fra modellens palet - agenten kan simpelthen ikke anmode om dem. Se [Oversigt over tilladte værktøjskald](#tools-overview).
- **Revisionsfelter på hver handling.** Modellen skal inkludere en begrundelse og en tillids-score. Begge vises i kørselstidlinjen og ved hver godkendelse. Se [Kørselsdetaljevisning](#run-detail-view).
- **EU DSA Artikel 17.** I EU-regionen er fuldautomatiske udelukkelser blokeret. Se [Overholdelse af EU DSA Artikel 17](#eu-dsa-compliance).
- **Ingen træning på dine data.** FastComments bruger leverandører, der ikke træner på dine prompts eller kommentarer.

### Hvor de passer ind ved siden af menneskelig moderation

Agenter og menneskelige moderatorer deler den samme kommentarsplatform: agenter udfører handlinger gennem de samme kanaler (godkend, markér som spam, udeluk, tildel mærke, fastgør, lås, skriv) og disse handlinger vises i de samme [Kommentarlog](/guide-moderation.html#comment-logs), på den samme [Moderationsside](/guide-moderation.html#moderate-comments-page) og i de samme underretningsstrømme. Agenter og mennesker ser hinandens arbejde og kan reagere på hinanden — moderatorhandlinger er i sig selv gyldige agentudløsere (se [Udløser: Moderator gennemgået kommentar](#trigger-moderator-reviewed) og relaterede).