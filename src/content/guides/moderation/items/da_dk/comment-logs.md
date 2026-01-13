FastComments registrerer automatisk detaljerede hændelser for hver kommentar for at skabe gennemsigtighed i moderationsbeslutninger og systemhandlinger. Disse logfiler hjælper dig med at forstå, hvorfor en kommentar blev godkendt, markeret som spam eller fik ændret sin status.

Du kan se kommentarslogfiler for individuelle kommentarer i Moderate Comments-dashboardet ved at vælge en specifik kommentar.

## Comment Log Events

Hver kommentar opretholder en log over hændelser, der sker i løbet af dens livscyklus. Nedenfor er typerne af hændelser, der registreres:

### Anonymization Events
- **Anonymized** - Kommentarens indhold blev fjernet og brugeren markeret som slettet

### Approval Events
- **ApprovedDueToPastComment** - Kommentar godkendt fordi brugeren tidligere har haft godkendte kommentarer
- **ApprovedIsAdmin** - Kommentar godkendt fordi brugeren er administrator
- **NotApprovedRequiresApproval** - Kommentar kræver manuel godkendelse

### Spam Detection Events
- **IsSpam** - Kommentar markeret som spam af detektionsmotoren
- **IsSpamDueToBadWords** - Kommentar markeret som spam på grund af bandeordsfilteret
- **IsSpamFromLLM** - Kommentar markeret som spam af AI/LLM-motoren
- **IsSpamRepeatComment** - Kommentar markeret som spam for at være gentagen
- **NotSpamIsOnlyImage** - Kommentar ikke markeret som spam fordi den kun indeholder billeder
- **NotSpamIsOnlyReacts** - Kommentar ikke markeret som spam fordi den kun indeholder reaktioner
- **NotSpamNoLinkOrMention** - Kommentar ikke markeret som spam på grund af ingen mistænkelige links eller omtaler
- **NotSpamPerfectTrustFactor** - Kommentar ikke markeret som spam på grund af høj brugertillid
- **NotSpamTooShort** - Kommentar ikke markeret som spam fordi den er for kort til at analysere
- **NotSpamSkipped** - Spamkontrol blev sprunget over
- **NotSpamFromEngine** - Kommentar vurderet ikke som spam af detektionsmotoren

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Bandordstjek mislykkedes
- **BadWordsFoundBadPhrase** - Bandordsfilter opdagede en upassende frase
- **BadWordsFoundBadWord** - Bandordsfilter opdagede et upassende ord
- **BadWordsNoDefinitionForLocale** - Ingen bandordsdefinitioner tilgængelige for kommentarens sprog

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Kommentar kræver verifikation, men brugeren er ikke i en verificeret session
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Kommentar kræver verifikation, men brugeren er endnu ikke verificeret
- **InVerifiedSession** - Brugeren, der poster kommentaren, er i en verificeret session
- **SentVerificationEmailNoSession** - Verifikations-e-mail sendt til uverificeret bruger
- **SentWelcomeEmail** - Velkomst-e-mail sendt til ny bruger

### Trust and Security Events
- **TrustFactorChanged** - Brugerens tillidsfaktor blev ændret
- **SpamFilterDisabledBecauseAdmin** - Spamfilter omgået for administratorbruger
- **TenantSpamFilterDisabled** - Spamfiltrering deaktiveret for hele tenant
- **RepeatCommentCheckIgnored** - Check for gentagne kommentarer blev ignoreret
- **UserIsAdmin** - Bruger identificeret som administrator
- **UserIsAdminParentTenant** - Bruger identificeret som parent-tenant administrator
- **UserIsAdminViaSSO** - Bruger identificeret som administrator via SSO
- **UserIsMod** - Bruger identificeret som moderator

### Comment Status Changes
- **ExpireStatusChanged** - Kommentarens udløbsstatus blev ændret
- **ReviewStatusChanged** - Kommentarens anmeldelsesstatus blev ændret
- **SpamStatusChanged** - Kommentarens spamstatus blev opdateret
- **ApproveStatusChanged** - Kommentarens godkendelsesstatus blev ændret
- **TextChanged** - Kommentarens tekstindhold blev redigeret
- **VotesChanged** - Kommentarens stemmetal blev opdateret
- **Flagged** - Kommentar blev flaget af brugere
- **UnFlagged** - Kommentarflags blev fjernet

### Moderation Actions
- **Pinned** - Kommentar blev fastgjort af moderator
- **UnPinned** - Kommentar blev af-fastgjort af moderator
- **RestoredFromAnonymized** - Kommentar blev gendannet fra anonymiseret tilstand

### Notification Events
- **CreatedNotifications** - Notifikationer blev oprettet for kommentaren
- **NotificationCreateFailure** - Fejl ved oprettelse af notifikationer
- **BadgeAwarded** - Bruger fik tildelt badge for kommentaren

### Publishing Events
- **PublishedLive** - Kommentar blev udgivet til live-abonnenter

### Integration Events
- **WebhookSynced** - Kommentar blev synkroniseret via webhook

### Spam Rule Events
- **SpamRuleMatch** - Kommentar matchede en brugerdefineret spamregel

## Accessing Comment Logs

Kommentarslogfiler genereres automatisk og gemmes sammen med hver kommentar. De giver værdifuld indsigt til:

- Forståelse af moderationsbeslutninger
- Fejlfinding af godkendelses-/spamproblemer
- Sporing af brugeradfærdsmønstre
- Revision af systemhandlinger

Disse logfiler hjælper med at opretholde gennemsigtighed i moderationsprocessen og assisterer med at finjustere adfærden i dit kommentarsystem.