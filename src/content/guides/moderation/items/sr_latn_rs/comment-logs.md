FastComments automatski prati detaljne događaje za svaki komentar kako bi obezbedio transparentnost u donošenju odluka o moderaciji i akcijama sistema. Ovi zapisi pomažu da razumete zašto je komentar odobren, označen kao spam ili mu je promenjen status.

Možete pregledati dnevnike komentara za pojedinačne komentare u kontrolnoj tabli Moderate Comments tako što ćete odabrati određeni komentar.

## Comment Log Events

Svaki komentar održava zapis događaja koji se dešavaju tokom njegovog životnog ciklusa. Ispod su tipovi događaja koji se prate:

### Anonymization Events
- **Anonymized** - Sadržaj komentara je obrisan i korisnik označen kao obrisan

### Approval Events
- **ApprovedDueToPastComment** - Komentar odobren jer je korisnik prethodno imao odobrene komentare
- **ApprovedIsAdmin** - Komentar odobren jer je korisnik administrator
- **NotApprovedRequiresApproval** - Komentar zahteva ručno odobrenje

### Spam Detection Events
- **IsSpam** - Komentar označen kao spam od strane mehanizma za detekciju
- **IsSpamDueToBadWords** - Komentar označen kao spam zbog filtera za nepristojne reči
- **IsSpamFromLLM** - Komentar označen kao spam od strane AI/LLM mehanizma
- **IsSpamRepeatComment** - Komentar označen kao spam zbog ponavljanja
- **NotSpamIsOnlyImage** - Komentar nije označen kao spam jer sadrži samo slike
- **NotSpamIsOnlyReacts** - Komentar nije označen kao spam jer sadrži samo reakcije
- **NotSpamNoLinkOrMention** - Komentar nije označen kao spam jer ne sadrži sumnjive linkove ili pominjanja
- **NotSpamPerfectTrustFactor** - Komentar nije označen kao spam zbog visokog poverenja korisnika
- **NotSpamTooShort** - Komentar nije označen kao spam jer je prekratak za analizu
- **NotSpamSkipped** - Provera spama je preskočena
- **NotSpamFromEngine** - Komentar utvrđen kao ne-spam od strane mehanizma za detekciju

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Provera filtera za nepristojne reči je naišla na grešku
- **BadWordsFoundBadPhrase** - Filter za nepristojne reči je detektovao neprikladnu frazu
- **BadWordsFoundBadWord** - Filter za nepristojne reči je detektovao neprikladnu reč
- **BadWordsNoDefinitionForLocale** - Nema definicija profaniteta dostupnih za jezik komentara

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahteva verifikaciju ali korisnik nije u verifikovanoj sesiji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahteva verifikaciju ali korisnik još nije verificiran
- **InVerifiedSession** - Korisnik koji objavljuje komentar je u verifikovanoj sesiji
- **SentVerificationEmailNoSession** - Verifikacioni email poslat neverifikovanom korisniku
- **SentWelcomeEmail** - Dobrodošli email poslat novom korisniku

### Trust and Security Events
- **TrustFactorChanged** - Faktor poverenja korisnika je izmenjen
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama zaobiđeno za administratorskog korisnika
- **TenantSpamFilterDisabled** - Filtriranje spama onemogućeno za ceo tenant
- **RepeatCommentCheckIgnored** - Provera za ponovljeni komentar je zaobiđena
- **UserIsAdmin** - Korisnik identifikovan kao administrator
- **UserIsAdminParentTenant** - Korisnik identifikovan kao administrator roditeljskog tena
- **UserIsAdminViaSSO** - Korisnik identifikovan kao administrator putem SSO
- **UserIsMod** - Korisnik identifikovan kao moderator

### Comment Status Changes
- **ExpireStatusChanged** - Status isteka komentara je izmenjen
- **ReviewStatusChanged** - Status pregleda komentara je promenjen
- **SpamStatusChanged** - Status spama komentara je ažuriran
- **ApproveStatusChanged** - Status odobrenja komentara je promenjen
- **TextChanged** - Tekst komentara je izmenjen
- **VotesChanged** - Broj glasova za komentar je ažuriran
- **Flagged** - Komentar je označen od strane korisnika
- **UnFlagged** - Oznake za komentar su uklonjene

### Moderation Actions
- **Pinned** - Komentar je zakačen (pinned) od strane moderatora
- **UnPinned** - Komentar je skinut sa vrha od strane moderatora
- **RestoredFromAnonymized** - Komentar je vraćen iz anonimnog stanja

### Notification Events
- **CreatedNotifications** - Obaveštenja su kreirana za komentar
- **NotificationCreateFailure** - Nije uspelo kreiranje obaveštenja
- **BadgeAwarded** - Korisniku je dodeljena značka za komentar

### Publishing Events
- **PublishedLive** - Komentar je objavljen pretplatnicima uživo

### Integration Events
- **WebhookSynced** - Komentar je sinhronizovan putem webhook-a

### Spam Rule Events
- **SpamRuleMatch** - Komentar je odgovarao prilagođenom pravilu za spam

## Accessing Comment Logs

Dnevnici komentara se automatski generišu i čuvaju uz svaki komentar. Oni pružaju vredne uvide za:

- Razumevanje odluka o moderaciji
- Otkrivanje i rešavanje problema sa odobravanjem/spamom
- Praćenje obrazaca ponašanja korisnika
- Reviziju akcija sistema

Ovi zapisi pomažu održavanju transparentnosti u procesu moderacije i olakšavaju fino podešavanje ponašanja vašeg sistema za komentare.