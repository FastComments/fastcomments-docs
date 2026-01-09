FastComments samodejno spremlja podrobne dogodke za vsak komentar, da zagotovi preglednost pri odločitvah moderiranja in dejanjih sistema. Ti zapisi vam pomagajo razumeti, zakaj je bil komentar odobren, označen kot vsiljena pošta (spam) ali zakaj se je spremenil njegov status.

Zapise komentarjev si lahko ogledate za posamezne komentarje na nadzorni plošči za moderiranje komentarjev tako, da izberete določen komentar.

## Dogodki v zapisu komentarja

Vsak komentar vzdržuje zapis dogodkov, ki se zgodijo med njegovim življenjskim ciklom. Spodaj so vrste dogodkov, ki se beležijo:

### Dogodki anonimizacije
- **Anonymized** - Vsebina komentarja je bila izpraznjena in uporabnik označen kot izbrisan

### Dogodki odobritve
- **ApprovedDueToPastComment** - Komentar odobren, ker je uporabnik imel prej odobrene komentarje
- **ApprovedIsAdmin** - Komentar odobren, ker je uporabnik skrbnik
- **NotApprovedRequiresApproval** - Komentar zahteva ročno odobritev

### Dogodki zaznavanja spama
- **IsSpam** - Komentar označen kot spam s strani mehanizma za zaznavanje
- **IsSpamDueToBadWords** - Komentar označen kot spam zaradi filtra za neprimerne besede
- **IsSpamFromLLM** - Komentar označen kot spam s strani AI/LLM mehanizma
- **IsSpamRepeatComment** - Komentar označen kot spam zaradi ponavljanja
- **NotSpamIsOnlyImage** - Komentar ni bil označen kot spam, ker vsebuje samo slike
- **NotSpamIsOnlyReacts** - Komentar ni bil označen kot spam, ker vsebuje samo reakcije
- **NotSpamNoLinkOrMention** - Komentar ni bil označen kot spam, ker ne vsebuje sumljivih povezav ali omemb
- **NotSpamPerfectTrustFactor** - Komentar ni bil označen kot spam zaradi visokega zaupanja uporabnika
- **NotSpamTooShort** - Komentar ni bil označen kot spam, ker je prekratek za analizo
- **NotSpamSkipped** - Preverjanje spama je bilo preskočeno
- **NotSpamFromEngine** - Komentar je bil kot ne-spam ocenjen s strani detekcijskega mehanizma

### Dogodki neprimernih besed/žaljivk
- **BadWordsCheckFailed** - Preverjanje filtra za neprimerne besede je naletelo na napako
- **BadWordsFoundBadPhrase** - Filter za neprimerne besede je zaznal neprimerno frazo
- **BadWordsFoundBadWord** - Filter za neprimerne besede je zaznal neprimerno besedo
- **BadWordsNoDefinitionForLocale** - Za jezik komentarja ni na voljo definicij za filter neprimernih besed

### Dogodki preverjanja uporabnika
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahteva preverjanje, vendar uporabnik ni v preverjeni seji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahteva preverjanje, vendar uporabnik še ni preverjen
- **InVerifiedSession** - Uporabnik, ki objavlja komentar, je v preverjeni seji
- **SentVerificationEmailNoSession** - Preveritveno e-poštno sporočilo poslano nepreverjenemu uporabniku
- **SentWelcomeEmail** - Pozdravno e-poštno sporočilo poslano novemu uporabniku

### Dogodki zaupanja in varnosti
- **TrustFactorChanged** - Zaupanje uporabnika je bilo spremenjeno
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama je bilo zaobšlo zaradi skrbnika
- **TenantSpamFilterDisabled** - Filtriranje spama onemogočeno za celotnega najemnika
- **RepeatCommentCheckIgnored** - Preverjanje ponavljajočih se komentarjev je bilo preskočeno
- **UserIsAdmin** - Uporabnik je prepoznan kot skrbnik
- **UserIsAdminParentTenant** - Uporabnik je prepoznan kot skrbnik nadrejenega najemnika
- **UserIsAdminViaSSO** - Uporabnik je prepoznan kot skrbnik preko SSO
- **UserIsMod** - Uporabnik je prepoznan kot moderator

### Spremembe statusa komentarja
- **ExpireStatusChanged** - Status poteka/izteka komentarja je bil spremenjen
- **ReviewStatusChanged** - Status pregleda komentarja je bil spremenjen
- **SpamStatusChanged** - Status spama komentarja je bil posodobljen
- **ApproveStatusChanged** - Status odobritve komentarja je bil spremenjen
- **TextChanged** - Besedilo komentarja je bilo urejeno
- **VotesChanged** - Število glasov za komentar je bilo posodobljeno
- **Flagged** - Komentar so označili uporabniki
- **UnFlagged** - Označbe komentarja so bile odstranjene

### Ukrepi moderiranja
- **Pinned** - Komentar je bil pritrjen s strani moderatorja
- **UnPinned** - Komentar je bil odpravljen iz pritrditve s strani moderatorja
- **RestoredFromAnonymized** - Komentar je bil obnovljen iz anonimiziranega stanja

### Dogodki obvestil
- **CreatedNotifications** - Za komentar so bila ustvarjena obvestila
- **NotificationCreateFailure** - Ustvarjanje obvestil ni uspelo
- **BadgeAwarded** - Uporabniku je bila dodeljena značka za komentar

### Dogodki objave
- **PublishedLive** - Komentar je bil objavljen za naročnike v živo

### Dogodki integracij
- **WebhookSynced** - Komentar je bil sinhroniziran preko webhooka

### Dogodki pravil spama
- **SpamRuleMatch** - Komentar je ustrezal po meri določeni pravili spama

## Dostop do zapisov komentarjev

Zapisi komentarjev so samodejno ustvarjeni in shranjeni pri vsakem komentarju. Nudijo vredne vpoglede za:

- Razumevanje odločitev pri moderiranju
- Odpravljanje napak pri odobritvi/težavah s spamom
- Sledenje vzorcem vedenja uporabnikov
- Revizijo dejanj sistema

Ti zapisi pomagajo ohranjati preglednost v procesu moderiranja in pomagajo pri izpopolnjevanju vedenja vašega sistema komentarjev.