FastComments sporer automatisk detaljerede hændelser for hver kommentar for at give gennemsigtighed i moderationsbeslutninger og systemhandlinger. Disse logfiler hjælper dig med at forstå, hvorfor en kommentar blev godkendt, markeret som spam, eller fik sin status ændret.

## Adgang til kommentarlogfiler

1. Naviger til siden **Moderate Comments** i dit FastComments-dashboard
2. Find den kommentar, du vil inspicere
3. Klik på knappen **View Logs** (ur-ikon) i kommentarens handlingslinje
4. En dialog vises, som viser den komplette historik over hændelser for den kommentar

Hver logpost viser:
- **When** - Tidsstemplet for hændelsen
- **Who** - Brugeren eller systemet, der udløste hændelsen (når relevant)
- **What** - Typen af handling eller hændelse
- **Details** - Yderligere kontekst såsom før/efter værdier, motor-navne eller relaterede data

## Kommentarloghændelser

Hver kommentar opretholder en log over hændelser, der forekommer i løbet af dens livscyklus. Nedenfor er typerne af hændelser, der spores:

### Anonymiseringshændelser
- **Anonymized** - Kommentarindholdet blev ryddet, og brugeren markeret som slettet
- **RestoredFromAnonymized** - Kommentaren blev gendannet fra anonymiseret tilstand

### Godkendelseshændelser
- **ApprovedDueToPastComment** - Kommentar godkendt fordi brugeren tidligere har godkendt kommentarer (inkluderer reference til den tidligere kommentar)
- **ApprovedIsAdmin** - Kommentar godkendt fordi brugeren er admin
- **NotApprovedRequiresApproval** - Kommentar kræver manuel godkendelse
- **NotApprovedLowTrustFactor** - Kommentar ikke godkendt på grund af lav bruger‑tillidsfaktor (inkluderer tillidsfaktor‑værdien)

### Profilkommentar-godkendelseshændelser
Disse hændelser gælder specifikt for kommentarer på brugerprofiler:
- **ApprovedProfileAutoApproveAll** - Profilkommentar auto-godkendt fordi profilens ejer har aktiveret auto‑godkendelse for alle kommentarer
- **ApprovedProfileTrusted** - Profilkommentar godkendt fordi kommentatoren er betroet (inkluderer reference til den kommentar, der etablerede tillid)
- **NotApprovedProfileManualApproveAll** - Profilkommentar kræver manuel godkendelse fordi profilens ejer har aktiveret manuel godkendelse
- **NotApprovedProfileNotTrusted** - Profilkommentar ikke godkendt fordi kommentatoren ikke er betroet
- **NotApprovedProfileNewUser** - Profilkommentar ikke godkendt fordi kommentatoren er en ny bruger

### Spam‑detekteringshændelser
- **IsSpam** - Kommentar markeret som spam af detektionsmotor (inkluderer hvilken motor der traf beslutningen)
- **IsSpamDueToBadWords** - Kommentar markeret som spam på grund af bandeordfilter
- **IsSpamFromLLM** - Kommentar markeret som spam af AI/LLM-motor (inkluderer motorens navn, svar og token‑antal)
- **IsSpamRepeatComment** - Kommentar markeret som spam for at være gentagende (inkluderer hvilken motor der opdagede det)
- **NotSpamIsOnlyImage** - Kommentar ikke markeret som spam fordi den kun indeholder billeder
- **NotSpamIsOnlyReacts** - Kommentar ikke markeret som spam fordi den kun indeholder reaktioner
- **NotSpamNoLinkOrMention** - Kommentar ikke markeret som spam på grund af ingen mistænkelige links eller nævnelser
- **NotSpamPerfectTrustFactor** - Kommentar ikke markeret som spam på grund af høj bruger‑tillid
- **NotSpamTooShort** - Kommentar ikke markeret som spam fordi den er for kort til at analysere
- **NotSpamSkipped** - Spam‑kontrol blev sprunget over
- **NotSpamFromEngine** - Kommentar bestemt som ikke‑spam af detektionsmotor (inkluderer motorens navn og tillidsfaktor)

### Bandeord/obskønitets‑hændelser
- **BadWordsCheckFailed** - Obskønitetsfilteret stødte på en fejl
- **BadWordsFoundBadPhrase** - Obskønitetsfilteret opdagede en upassende sætning (inkluderer sætningen)
- **BadWordsFoundBadWord** - Obskønitetsfilteret opdagede et upassende ord (inkluderer ordet)
- **BadWordsNoDefinitionForLocale** - Ingen obskønitetsdefinitioner tilgængelige for kommentarens sprog (inkluderer lokalet)

### Bruger‑verifikationshændelser
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Kommentar kræver verifikation, men brugeren er ikke i en verificeret session
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Kommentar kræver verifikation, men brugeren er endnu ikke verificeret
- **InVerifiedSession** - Brugeren, der poster kommentaren, er i en verificeret session
- **SentVerificationEmailNoSession** - Verifikations‑email sendt til en ikke‑verificeret bruger
- **SentWelcomeEmail** - Velkomst‑email sendt til ny bruger

### Tillids‑ og sikkerhedshændelser
- **TrustFactorChanged** - Brugerens tillidsfaktor blev ændret (inkluderer før‑ og efter‑værdier)
- **SpamFilterDisabledBecauseAdmin** - Spam‑filtrering omgået for admin‑bruger
- **TenantSpamFilterDisabled** - Spam‑filtrering deaktiveret for hele lejer
- **RepeatCommentCheckIgnored** - Gentagelses‑kommentar‑tjek blev ignoreret (inkluderer årsagen)
- **UserIsAdmin** - Bruger identificeret som admin
- **UserIsAdminParentTenant** - Bruger identificeret som over‑lejer‑admin
- **UserIsAdminViaSSO** - Bruger identificeret som admin via SSO
- **UserIsMod** - Bruger identificeret som moderator

### Kommentarstatus‑ændringer
Statusændrings‑hændelser inkluderer før‑ og efter‑værdier samt brugeren, der foretog ændringen:
- **ExpireStatusChanged** - Kommentarens udløbsstatus blev ændret
- **ReviewStatusChanged** - Kommentarens gennemgangsstatus blev ændret
- **SpamStatusChanged** - Kommentarens spam‑status blev opdateret
- **ApproveStatusChanged** - Kommentarens godkendelsesstatus blev ændret
- **TextChanged** - Kommentarens tekstindhold blev redigeret (inkluderer før‑ og efter‑tekst)
- **VotesChanged** - Kommentarens stemmetal blev opdateret (inkluderer detaljeret stemmefordeling)
- **Flagged** - Kommentar blev flaget af brugere
- **UnFlagged** - Kommentarens flag blev fjernet

### Moderations‑handlinger
- **Pinned** - Kommentar blev fastgjort af moderator (inkluderer hvem der fastgjorde den)
- **UnPinned** - Kommentar blev løsnet af moderator (inkluderer hvem der løsnet den)

### Notifikations‑hændelser
- **CreatedNotifications** - Notifikationer blev oprettet for kommentaren (inkluderer antal notifikationer)
- **NotificationCreateFailure** - Fejl ved oprettelse af notifikationer
- **BadgeAwarded** - Bruger‑badge blev tildelt for kommentaren (inkluderer badge‑navn)

### Nævning‑ og svar‑notifikations‑hændelser
Disse hændelser angiver personen, der ville modtage e‑mailen eller notifikationen. Når intet blev sendt, angiver kolonnen Detaljer hvorfor.
- **MentionEmailSent** - En bruger, der blev nævnt i kommentaren, blev sendt e‑mail
- **MentionEmailSkipped** - En nævnt bruger blev ikke sendt e‑mail (inkluderer årsagen)
- **MentionHeldForApproval** - Nævnings‑e‑mailen afventer indtil kommentaren er godkendt
- **MentionNotificationCreated** - En nævnt bruger modtog en in‑app‑notifikation
- **MentionNotificationSkipped** - En nævnt bruger modtog ikke en in‑app‑notifikation (inkluderer årsagen)
- **ReplyEmailSent** - Forfatteren af den kommentar, der blev svaret på, blev sendt e‑mail om dette svar
- **ReplyEmailSkipped** - Forfatteren af den kommentar, der blev svaret på, blev ikke sendt e‑mail (inkluderer årsagen)
- **ReplyNotificationSkipped** - Forfatteren af den kommentar, der blev svaret på, modtog ikke en in‑app‑notifikation (inkluderer årsagen)

Årsager vist når en e‑mail eller notifikation ikke blev sendt:
- Brugeren findes ikke længere, eller har ingen e‑mailadresse
- Brugeren har slået e‑mail‑notifikationer fra, eller har slået notifikationer fra for den tråd
- En af de to brugere har blokeret den anden
- Brugerne er ikke i nogen af de samme SSO‑grupper
- Brugerens e‑mailadresse er på undertrykkelseslisten efter et bounce eller spam‑klage (se [Email Suppression Management](/guide-notifications.html#email-suppression-management))
- Brugerens e‑mailadresse er på example.com, som ikke kan modtage e‑mail
- Kommentaren blev markeret som spam, slettet, eller ikke godkendt inden for 7 dage
- Kommentaren, der blev svaret på, blev efterladt anonymt
- Brugeren svarede på sin egen kommentar
- Brugeren blev nævnt i svaret, så de fik nævnings‑e‑mailen i stedet for en svar‑e‑mail
- Brugeren havde allerede en svar‑notifikation for kommentaren
- Afsendelse mislykkedes 5 gange

Hvis leveringen fejler eller rammer en afsendelsesgrænse, bliver e‑mailen sat i kø til at blive forsøgt igen, og logposten angiver dette.

### Publicerings‑hændelser
- **PublishedLive** - Kommentar blev offentliggjort til live‑abonnenter (inkluderer antal abonnenter)

### Integrations‑hændelser
- **WebhookSynced** - Kommentar blev synkroniseret via webhook

### Spam‑regel‑hændelser
- **SpamRuleMatch** - Kommentar matchede en brugerdefineret spam‑regel (inkluderer regeldetaljer)

### Lokalisering‑hændelser
- **LocaleDetectedFromText** - Sproglokale blev automatisk opdaget fra kommentarens tekst (inkluderer detekteret sprog og lokale)

## Anvendelsestilfælde for kommentarlogfiler

Kommentarlogfiler genereres automatisk og gemmes med hver kommentar. De giver værdifulde indsigter til:
- **Understanding moderation decisions** - Se præcis hvorfor en kommentar blev godkendt, holdt til gennemgang, eller markeret som spam
- **Debugging approval/spam issues** - Spor beslutningslogikken når kommentarer ikke opfører sig som forventet
- **Tracking user behavior patterns** - Overvåg ændringer i tillidsfaktor og verifikationsstatus
- **Auditing moderator actions** - Gennemgå hvilke handlinger moderatorer har foretaget på specifikke kommentarer
- **Investigating spam filter effectiveness** - Se hvilke detektionsmotorer der fanger spam, og hvilke der ikke gør
- **Troubleshooting integrations** - Verificer webhook‑synkroniseringer og notifikationslevering

Disse logfiler hjælper med at opretholde gennemsigtighed i moderationsprocessen og assisterer med at finjustere din kommentarsystems adfærd.