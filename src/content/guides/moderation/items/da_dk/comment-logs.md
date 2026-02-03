FastComments registrerer automatisk detaljerede hændelser for hver kommentar for at give gennemsigtighed i moderationsbeslutninger og systemhandlinger. Disse logfiler hjælper dig med at forstå, hvorfor en kommentar blev godkendt, markeret som spam eller fik ændret status.

## Adgang til kommentarlogfiler

For at se logfilerne for en specifik kommentar:

1. Gå til siden **Moderate Comments** i dit FastComments-dashboard
2. Find den kommentar, du vil undersøge
3. Klik på knappen **View Logs** (ur-ikonet) i kommentarens handlingslinje
4. En dialogboks vises med den komplette historik af hændelser for den kommentar

Hver logpost viser:
- **Hvornår** - Tidsstemplet for hændelsen
- **Hvem** - Brugeren eller systemet, der udløste hændelsen (hvis relevant)
- **Hvad** - Typen af handling eller hændelse
- **Detaljer** - Yderligere kontekst såsom før/efter værdier, navne på engines eller relaterede data

## Comment Log Events

Hver kommentar bevarer en log over hændelser, der opstår i løbet af dens livscyklus. Nedenfor er typerne af hændelser, der spores:

### Anonymization Events
- **Anonymized** - Kommentarindhold blev ryddet og brugeren markeret som slettet
- **RestoredFromAnonymized** - Kommentar blev gendannet fra anonymiseret tilstand

### Approval Events
- **ApprovedDueToPastComment** - Kommentar godkendt, fordi brugeren tidligere har haft godkendte kommentarer (inkluderer reference til den tidligere kommentar)
- **ApprovedIsAdmin** - Kommentar godkendt, fordi brugeren er administrator
- **NotApprovedRequiresApproval** - Kommentar kræver manuel godkendelse
- **NotApprovedLowTrustFactor** - Kommentar ikke godkendt på grund af lav brugertillidsfaktor (inkluderer tillidsfaktorens værdi)

### Profile Comment Approval Events

Disse hændelser gælder specifikt for kommentarer på brugerprofiler:

- **ApprovedProfileAutoApproveAll** - Profilkommentar auto-godkendt, fordi profilejeren har aktiveret automatisk godkendelse for alle kommentarer
- **ApprovedProfileTrusted** - Profilkommentar godkendt, fordi kommentatoren er betroet (inkluderer reference til den kommentar, der etablerede tillid)
- **NotApprovedProfileManualApproveAll** - Profilkommentar kræver manuel godkendelse, fordi profilejeren har aktiveret manuel godkendelse
- **NotApprovedProfileNotTrusted** - Profilkommentar ikke godkendt, fordi kommentatoren ikke er betroet
- **NotApprovedProfileNewUser** - Profilkommentar ikke godkendt, fordi kommentatoren er en ny bruger

### Spam Detection Events
- **IsSpam** - Kommentar markeret som spam af detekteringsengine (inkluderer hvilken engine der traf beslutningen)
- **IsSpamDueToBadWords** - Kommentar markeret som spam på grund af profanity-filter
- **IsSpamFromLLM** - Kommentar markeret som spam af AI/LLM-engine (inkluderer engine-navn, svar og tokenantal)
- **IsSpamRepeatComment** - Kommentar markeret som spam for at være gentagende (inkluderer hvilken engine der opdagede det)
- **NotSpamIsOnlyImage** - Kommentar ikke markeret som spam, fordi den kun indeholder billeder
- **NotSpamIsOnlyReacts** - Kommentar ikke markeret som spam, fordi den kun indeholder reaktioner
- **NotSpamNoLinkOrMention** - Kommentar ikke markeret som spam på grund af ingen mistænkelige links eller mentions
- **NotSpamPerfectTrustFactor** - Kommentar ikke markeret som spam på grund af høj brugertillid
- **NotSpamTooShort** - Kommentar ikke markeret som spam, fordi den er for kort til at analysere
- **NotSpamSkipped** - Spamcheck blev sprunget over
- **NotSpamFromEngine** - Kommentar vurderet ikke som spam af detekteringsengine (inkluderer engine-navn og tillidsfaktor)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Profanity-filter tjek mislykkedes
- **BadWordsFoundBadPhrase** - Profanity-filter opdagede en upassende frase (inkluderer frasen)
- **BadWordsFoundBadWord** - Profanity-filter opdagede et upassende ord (inkluderer ordet)
- **BadWordsNoDefinitionForLocale** - Ingen profanity-definitioner tilgængelige for kommentarens sprog (inkluderer locale)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Kommentar kræver verifikation, men brugeren er ikke i en verificeret session
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Kommentar kræver verifikation, men brugeren er endnu ikke verificeret
- **InVerifiedSession** - Bruger, der poster kommentar, er i en verificeret session
- **SentVerificationEmailNoSession** - Verifikationsmail sendt til uverificeret bruger
- **SentWelcomeEmail** - Velkomstmail sendt til ny bruger

### Trust and Security Events
- **TrustFactorChanged** - Brugerens tillidsfaktor blev ændret (inkluderer før og efter værdier)
- **SpamFilterDisabledBecauseAdmin** - Spamfiltrering omgået for administratorbruger
- **TenantSpamFilterDisabled** - Spamfiltrering deaktiveret for hele tenant
- **RepeatCommentCheckIgnored** - Repeat-kommentartjek blev omgået (inkluderer årsagen)
- **UserIsAdmin** - Bruger identificeret som administrator
- **UserIsAdminParentTenant** - Bruger identificeret som parent-tenant administrator
- **UserIsAdminViaSSO** - Bruger identificeret som administrator via SSO
- **UserIsMod** - Bruger identificeret som moderator

### Comment Status Changes

Statusændringshændelser inkluderer før- og efterværdier samt brugeren, der foretog ændringen:

- **ExpireStatusChanged** - Kommentarens forfaldsstatus blev ændret
- **ReviewStatusChanged** - Kommentarens gennemgangsstatus blev ændret
- **SpamStatusChanged** - Kommentarens spamstatus blev opdateret
- **ApproveStatusChanged** - Kommentarens godkendelsesstatus blev ændret
- **TextChanged** - Kommentarens tekstindhold blev redigeret (inkluderer før- og eftertekst)
- **VotesChanged** - Kommentarens stemmetal blev opdateret (inkluderer detaljeret stemmefordeling)
- **Flagged** - Kommentar blev markeret af brugere
- **UnFlagged** - Kommentarflag blev fjernet

### Moderation Actions
- **Pinned** - Kommentar blev fastgjort af moderator (inkluderer hvem der fastgjorde den)
- **UnPinned** - Kommentar blev af-fastgjort af moderator (inkluderer hvem der fjernede fastgørelsen)

### Notification Events
- **CreatedNotifications** - Notifikationer blev oprettet for kommentaren (inkluderer antal notifikationer)
- **NotificationCreateFailure** - Failed at oprette notifikationer
- **BadgeAwarded** - Brugermærke blev tildelt for kommentaren (inkluderer mærkens navn)

### Publishing Events
- **PublishedLive** - Kommentar blev publiceret til live-abonnenter (inkluderer abonnentantal)

### Integration Events
- **WebhookSynced** - Kommentar blev synkroniseret via webhook

### Spam Rule Events
- **SpamRuleMatch** - Kommentar matchede en brugerdefineret spamregel (inkluderer regeloplysninger)

### Localization Events
- **LocaleDetectedFromText** - Sprog/locale blev automatisk detekteret ud fra kommentarens tekst (inkluderer detekteret sprog og locale)

## Brugsscenarier for kommentarlogfiler

Kommentarlogs genereres automatisk og gemmes sammen med hver kommentar. De giver værdifuld indsigt til:

- **Forståelse af moderationsbeslutninger** - Se præcis, hvorfor en kommentar blev godkendt, holdt til gennemgang eller markeret som spam
- **Fejlfinding af godkendelses-/spamproblemer** - Spor beslutningslogikken gennem, når kommentarer ikke opfører sig som forventet
- **Overvågning af brugeradfærdsmønstre** - Følg ændringer i tillidsfaktor og verifikationsstatus
- **Revision af moderatorhandlinger** - Gennemgå hvilke handlinger moderatorer har foretaget på specifikke kommentarer
- **Undersøgelse af spamfilters effektivitet** - Se hvilke detekteringsengines der fanger spam og hvilke der ikke gør
- **Fejlfinding af integrationer** - Verificer webhook-synkroniseringer og levering af notifikationer

Disse logfiler hjælper med at opretholde gennemsigtighed i moderationsprocessen og assisterer med at finjustere adfærden i dit kommentarsystems opsætning.