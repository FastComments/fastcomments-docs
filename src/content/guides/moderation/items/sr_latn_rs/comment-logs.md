FastComments automatski prati detaljne događaje za svaki komentar kako bi obezbedio transparentnost u odlukama o moderisanju i sistemskim akcijama. Ovi zapisi pomažu da shvatite zašto je komentar odobren, označen kao spam ili mu je promenjen status.

## Pristupanje zapisima komentara

Da biste videli zapise za određeni komentar:

1. Idite na stranicu za moderisanje komentara u vašem FastComments kontrolnom panelu
2. Pronađite komentar koji želite da pregledate
3. Kliknite na dugme **Prikaži zapise** (ikona sata) u traci sa akcijama komentara
4. Pojaviće se dijalog koji prikazuje kompletno istoriju događaja za taj komentar

Svaki zapis prikazuje:
- **Kada** - Vremenska oznaka događaja
- **Ko** - Korisnik ili sistem koji je pokrenuo događaj (kada je primenljivo)
- **Šta** - Tip akcije ili događaja
- **Detalji** - Dodatni kontekst kao što su vrednosti pre/posle, imena engine-a ili povezani podaci

## Događaji zapisnika komentara

Svaki komentar održava zapis događaja koji se dešavaju tokom njegovog životnog ciklusa. Ispod su tipovi događaja koji se prate:

### Događaji anonimizacije
- **Anonymized** - Comment content was cleared and user marked as deleted
- **RestoredFromAnonymized** - Comment was restored from anonymized state

### Događaji odobravanja
- **ApprovedDueToPastComment** - Comment approved because user has previously approved comments (includes reference to the past comment)
- **ApprovedIsAdmin** - Comment approved because user is an admin
- **NotApprovedRequiresApproval** - Comment requires manual approval
- **NotApprovedLowTrustFactor** - Comment not approved due to low user trust factor (includes the trust factor value)

### Događaji odobravanja komentara na profilima

Ovi događaji se primenjuju specifično na komentare na korisničkim profilima:

- **ApprovedProfileAutoApproveAll** - Profile comment auto-approved because the profile owner has enabled auto-approve for all comments
- **ApprovedProfileTrusted** - Profile comment approved because the commenter is trusted (includes reference to the comment that established trust)
- **NotApprovedProfileManualApproveAll** - Profile comment requires manual approval because the profile owner has enabled manual approval
- **NotApprovedProfileNotTrusted** - Profile comment not approved because the commenter is not trusted
- **NotApprovedProfileNewUser** - Profile comment not approved because the commenter is a new user

### Događaji detekcije spama
- **IsSpam** - Comment flagged as spam by detection engine (includes which engine made the decision)
- **IsSpamDueToBadWords** - Comment flagged as spam due to profanity filter
- **IsSpamFromLLM** - Comment flagged as spam by AI/LLM engine (includes engine name, response, and token count)
- **IsSpamRepeatComment** - Comment flagged as spam for being repetitive (includes which engine detected it)
- **NotSpamIsOnlyImage** - Comment not flagged as spam because it only contains images
- **NotSpamIsOnlyReacts** - Comment not flagged as spam because it only contains reactions
- **NotSpamNoLinkOrMention** - Comment not flagged as spam due to no suspicious links or mentions
- **NotSpamPerfectTrustFactor** - Comment not flagged as spam due to high user trust
- **NotSpamTooShort** - Comment not flagged as spam because it's too short to analyze
- **NotSpamSkipped** - Spam check was skipped
- **NotSpamFromEngine** - Comment determined not spam by detection engine (includes engine name and trust factor)

### Događaji vezani za psovke/uvredljiv sadržaj
- **BadWordsCheckFailed** - Profanity filter check encountered an error
- **BadWordsFoundBadPhrase** - Profanity filter detected inappropriate phrase (includes the phrase)
- **BadWordsFoundBadWord** - Profanity filter detected inappropriate word (includes the word)
- **BadWordsNoDefinitionForLocale** - No profanity definitions available for comment language (includes the locale)

### Događaji verifikacije korisnika
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Comment requires verification but user not in verified session
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Comment requires verification but user not yet verified
- **InVerifiedSession** - User posting comment is in a verified session
- **SentVerificationEmailNoSession** - Verification email sent to unverified user
- **SentWelcomeEmail** - Welcome email sent to new user

### Događaji poverenja i bezbednosti
- **TrustFactorChanged** - User's trust factor was modified (includes before and after values)
- **SpamFilterDisabledBecauseAdmin** - Spam filtering bypassed for admin user
- **TenantSpamFilterDisabled** - Spam filtering disabled for entire tenant
- **RepeatCommentCheckIgnored** - Repeat comment check was bypassed (includes the reason)
- **UserIsAdmin** - User identified as admin
- **UserIsAdminParentTenant** - User identified as parent tenant admin
- **UserIsAdminViaSSO** - User identified as admin via SSO
- **UserIsMod** - User identified as moderator

### Promene statusa komentara

Događaji promene statusa uključuju vrednosti pre i posle, plus korisnika koji je izvršio promenu:

- **ExpireStatusChanged** - Comment expiration status was modified
- **ReviewStatusChanged** - Comment review status was changed
- **SpamStatusChanged** - Comment spam status was updated
- **ApproveStatusChanged** - Comment approval status was changed
- **TextChanged** - Comment text content was edited (includes before and after text)
- **VotesChanged** - Comment vote counts were updated (includes detailed vote breakdown)
- **Flagged** - Comment was flagged by users
- **UnFlagged** - Comment flags were removed

### Akcije moderacije
- **Pinned** - Comment was pinned by moderator (includes who pinned it)
- **UnPinned** - Comment was unpinned by moderator (includes who unpinned it)

### Događaji notifikacija
- **CreatedNotifications** - Notifications were created for comment (includes notification count)
- **NotificationCreateFailure** - Failed to create notifications
- **BadgeAwarded** - User badge was awarded for comment (includes badge name)

### Događaji objavljivanja
- **PublishedLive** - Comment was published to live subscribers (includes subscriber count)

### Događaji integracije
- **WebhookSynced** - Comment was synchronized via webhook

### Događaji pravila protiv spama
- **SpamRuleMatch** - Comment matched a custom spam rule (includes rule details)

### Događaji lokalizacije
- **LocaleDetectedFromText** - Language locale was automatically detected from comment text (includes detected language and locale)

## Upotrebe zapisnika komentara

Zapisnici komentara se automatski generišu i čuvaju uz svaki komentar. Oni pružaju vredne uvide za:

- **Razumevanje odluka o moderisanju** - Tačno vidite zašto je komentar odobren, zadržan na pregledu ili označen kao spam
- **Otklanjanje grešaka u vezi sa odobrenjem/spamom** - Pratite logiku odluke kada komentari ne funkcionišu očekivano
- **Praćenje obrazaca ponašanja korisnika** - Pratite promene faktora poverenja i status verifikacije
- **Reviziju akcija moderatora** - Pregledajte koje su akcije moderatori preduzeli nad određenim komentarima
- **Istragu efikasnosti spam filtera** - Vidite koji engine-i detektuju spam, a koji ne
- **Otklanjanje problema sa integracijama** - Proverite webhook sinhronizacije i isporuku notifikacija

Ovi zapisi pomažu u održavanju transparentnosti procesa moderisanja i olakšavaju fino podešavanje ponašanja vašeg sistema komentarisanja.