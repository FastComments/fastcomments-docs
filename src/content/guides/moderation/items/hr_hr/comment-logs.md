FastComments automatski bilježi detaljne događaje za svaki komentar kako bi pružio transparentnost u donošenju odluka o moderiranju i radnjama sustava. Ti zapisi pomažu vam razumjeti zašto je komentar odobren, označen kao spam ili mu je promijenjen status.

Možete pregledati zapise komentara za pojedinačne komentare na nadzornoj ploči za moderiranje komentara odabirom određenog komentara.

## Comment Log Events

Svaki komentar vodi zapis događaja koji se pojavljuju tijekom njegovog životnog ciklusa. Ispod su vrste događaja koje se prate:

### Anonymization Events
- **Anonymized** - Sadržaj komentara je obrisan i korisnik označen kao izbrisan

### Approval Events
- **ApprovedDueToPastComment** - Komentar odobren jer je korisnik ranije imao odobrene komentare
- **ApprovedIsAdmin** - Komentar odobren jer je korisnik administrator
- **NotApprovedRequiresApproval** - Komentar zahtijeva ručno odobrenje

### Spam Detection Events
- **IsSpam** - Komentar označen kao spam od strane mehanizma za otkrivanje
- **IsSpamDueToBadWords** - Komentar označen kao spam zbog filtra psovki
- **IsSpamFromLLM** - Komentar označen kao spam od strane AI/LLM mehanizma
- **IsSpamRepeatComment** - Komentar označen kao spam zbog ponavljanja
- **NotSpamIsOnlyImage** - Komentar nije označen kao spam jer sadrži samo slike
- **NotSpamIsOnlyReacts** - Komentar nije označen kao spam jer sadrži samo reakcije
- **NotSpamNoLinkOrMention** - Komentar nije označen kao spam jer ne sadrži sumnjive poveznice ili spominjanja
- **NotSpamPerfectTrustFactor** - Komentar nije označen kao spam zbog visokog stupnja povjerenja korisnika
- **NotSpamTooShort** - Komentar nije označen kao spam jer je prekratak za analizu
- **NotSpamSkipped** - Provjera spama je preskočena
- **NotSpamFromEngine** - Mehanizam za otkrivanje utvrdio da komentar nije spam

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Provjera filtra psovki je naišla na pogrešku
- **BadWordsFoundBadPhrase** - Filtracija psovki otkrila neprimjerenu frazu
- **BadWordsFoundBadWord** - Filtracija psovki otkrila neprimjerenu riječ
- **BadWordsNoDefinitionForLocale** - Nema definicija psovki dostupnih za jezik komentara

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahtijeva verifikaciju, ali korisnik nije u verificiranoj sesiji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahtijeva verifikaciju, ali korisnik još nije verificiran
- **InVerifiedSession** - Korisnik koji objavljuje komentar je u verificiranoj sesiji
- **SentVerificationEmailNoSession** - Poslan verifikacijski e-mail neverificiranom korisniku
- **SentWelcomeEmail** - Poslan dobrodošli e-mail novom korisniku

### Trust and Security Events
- **TrustFactorChanged** - Faktor povjerenja korisnika je promijenjen
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama zaobiđeno za administratorskog korisnika
- **TenantSpamFilterDisabled** - Filtriranje spama onemogućeno za cijeli tenant
- **RepeatCommentCheckIgnored** - Provjera ponovljenog komentara je zaobiđena
- **UserIsAdmin** - Korisnik identificiran kao administrator
- **UserIsAdminParentTenant** - Korisnik identificiran kao administrator roditeljskog tenanta
- **UserIsAdminViaSSO** - Korisnik identificiran kao administrator putem SSO
- **UserIsMod** - Korisnik identificiran kao moderator

### Comment Status Changes
- **ExpireStatusChanged** - Status isteka komentara je promijenjen
- **ReviewStatusChanged** - Status pregleda komentara je promijenjen
- **SpamStatusChanged** - Status spama komentara je ažuriran
- **ApproveStatusChanged** - Status odobrenja komentara je promijenjen
- **TextChanged** - Tekst komentara je uređivan
- **VotesChanged** - Broj glasova za komentar je ažuriran
- **Flagged** - Komentar je prijavljen od strane korisnika
- **UnFlagged** - Oznake (prijave) komentara su uklonjene

### Moderation Actions
- **Pinned** - Komentar je prikvačen od strane moderatora
- **UnPinned** - Komentar je uklonjen iz prikvačenih od strane moderatora
- **RestoredFromAnonymized** - Komentar je vraćen iz anonimiziranog stanja

### Notification Events
- **CreatedNotifications** - Obavijesti su stvorene za komentar
- **NotificationCreateFailure** - Neuspjelo stvaranje obavijesti
- **BadgeAwarded** - Korisniku je dodijeljena značka za komentar

### Publishing Events
- **PublishedLive** - Komentar je objavljen uživo pretplatnicima

### Integration Events
- **WebhookSynced** - Komentar je sinkroniziran putem webhooka

### Spam Rule Events
- **SpamRuleMatch** - Komentar je odgovarao prilagođenom pravilu spama

## Accessing Comment Logs

Zapisi komentara se automatski generiraju i pohranjuju uz svaki komentar. Oni pružaju vrijedne uvide za:

- Razumijevanje odluka o moderiranju
- Otklanjanje pogrešaka povezanih s odobravanjima/spamom
- Praćenje obrazaca ponašanja korisnika
- Reviziju radnji sustava

Ti zapisi pomažu održati transparentnost u procesu moderiranja i pomažu u finoj prilagodbi ponašanja vašeg sustava komentiranja.