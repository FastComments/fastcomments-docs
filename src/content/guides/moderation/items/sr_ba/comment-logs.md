FastComments automatski prati detaljne događaje za svaki komentar kako bi obezbijedio transparentnost u odlukama moderiranja i sistemskim akcijama. Ovi zapisi pomažu da razumijete zašto je komentar odobren, označen kao spam, ili mu je promijenjen status.

## Pristup zapisima komentara

Da biste prikazali zapise za određeni komentar:

1. Idite na stranicu **Moderate Comments** u vašem FastComments kontrolnom panelu
2. Pronađite komentar koji želite pregledati
3. Kliknite na dugme **View Logs** (ikonica sata) u traci sa akcijama komentara
4. Pojaviće se dijalog koji prikazuje kompletnu historiju događaja za taj komentar

Svaki zapis prikazuje:
- **When** - Vremenska oznaka događaja
- **Who** - Korisnik ili sistem koji je pokrenuo događaj (kada je primjenjivo)
- **What** - Tip akcije ili događaja
- **Details** - Dodatni kontekst kao što su vrijednosti prije/poslije, nazivi engine-a, ili povezani podaci

## Događaji u zapisima komentara

Svaki komentar održava zapis događaja koji se dešavaju tokom njegovog životnog ciklusa. Ispod su tipovi događaja koji se prate:

### Anonymization Events
- **Anonymized** - Sadržaj komentara je izbrisan i korisnik je označen kao obrisan
- **RestoredFromAnonymized** - Komentar je vraćen iz anonimizovanog stanja

### Approval Events
- **ApprovedDueToPastComment** - Komentar odobren jer korisnik ima ranije odobrene komentare (uključuje referencu na prethodni komentar)
- **ApprovedIsAdmin** - Komentar odobren jer je korisnik administrator
- **NotApprovedRequiresApproval** - Komentar zahtijeva ručno odobrenje
- **NotApprovedLowTrustFactor** - Komentar nije odobren zbog niskog faktora povjerenja korisnika (uključuje vrijednost faktora povjerenja)

### Profile Comment Approval Events

Ovi događaji se odnose posebno na komentare na korisničkim profilima:

- **ApprovedProfileAutoApproveAll** - Komentar na profilu automatski odobren jer je vlasnik profila omogućio automatsko odobravanje svih komentara
- **ApprovedProfileTrusted** - Komentar na profilu odobren jer je komentator pouzdan (uključuje referencu na komentar koji je uspostavio povjerenje)
- **NotApprovedProfileManualApproveAll** - Komentar na profilu zahtijeva ručno odobrenje jer je vlasnik profila omogućio ručno odobravanje
- **NotApprovedProfileNotTrusted** - Komentar na profilu nije odobren jer komentator nije pouzdan
- **NotApprovedProfileNewUser** - Komentar na profilu nije odobren jer je komentator novi korisnik

### Spam Detection Events
- **IsSpam** - Komentar označen kao spam od strane detekcionog engine-a (uključuje koji engine je donio odluku)
- **IsSpamDueToBadWords** - Komentar označen kao spam zbog filtera psovki
- **IsSpamFromLLM** - Komentar označen kao spam od strane AI/LLM engine-a (uključuje naziv engine-a, odgovor i broj tokena)
- **IsSpamRepeatComment** - Komentar označen kao spam zbog ponavljanja (uključuje koji engine je detektovao)
- **NotSpamIsOnlyImage** - Komentar nije označen kao spam jer sadrži samo slike
- **NotSpamIsOnlyReacts** - Komentar nije označen kao spam jer sadrži samo reakcije
- **NotSpamNoLinkOrMention** - Komentar nije označen kao spam zbog izostanka sumnjivih linkova ili spominjanja
- **NotSpamPerfectTrustFactor** - Komentar nije označen kao spam zbog visokog korisničkog faktora povjerenja
- **NotSpamTooShort** - Komentar nije označen kao spam jer je prekratak za analizu
- **NotSpamSkipped** - Provjera spama je preskočena
- **NotSpamFromEngine** - Komentar je od strane detekcionog engine-a utvrđen kao ne-spam (uključuje naziv engine-a i faktor povjerenja)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Provjera filtera psovki je naišla na grešku
- **BadWordsFoundBadPhrase** - Filter psovki je detektovao neprikladnu frazu (uključuje frazu)
- **BadWordsFoundBadWord** - Filter psovki je detektovao neprikladnu riječ (uključuje riječ)
- **BadWordsNoDefinitionForLocale** - Nema definicija psovki za jezik komentara (uključuje lokal)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahtijeva verifikaciju ali korisnik nije u verificiranoj sesiji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahtijeva verifikaciju ali korisnik još nije verificiran
- **InVerifiedSession** - Korisnik koji postavlja komentar je u verificiranoj sesiji
- **SentVerificationEmailNoSession** - Poslat je email za verifikaciju neverifikovanom korisniku
- **SentWelcomeEmail** - Poslat je email dobrodošlice novom korisniku

### Trust and Security Events
- **TrustFactorChanged** - Faktor povjerenja korisnika je izmijenjen (uključuje vrijednosti prije i poslije)
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama zaobiđeno zbog administratorskog korisnika
- **TenantSpamFilterDisabled** - Filtriranje spama onemogućeno za cijeli tenant
- **RepeatCommentCheckIgnored** - Provjera ponovljenih komentara je zaobiđena (uključuje razlog)
- **UserIsAdmin** - Korisnik identificiran kao administrator
- **UserIsAdminParentTenant** - Korisnik identificiran kao administrator parent tenanta
- **UserIsAdminViaSSO** - Korisnik identificiran kao administrator putem SSO
- **UserIsMod** - Korisnik identificiran kao moderator

### Comment Status Changes

Događaji promjene statusa uključuju vrijednosti prije i poslije, plus korisnika koji je izvršio promjenu:

- **ExpireStatusChanged** - Status isteka komentara je izmijenjen
- **ReviewStatusChanged** - Status pregleda komentara je promijenjen
- **SpamStatusChanged** - Status spama komentara je ažuriran
- **ApproveStatusChanged** - Status odobrenja komentara je promijenjen
- **TextChanged** - Tekst komentara je uređen (uključuje tekst prije i poslije)
- **VotesChanged** - Broj glasova komentara je ažuriran (uključuje detaljno razlaganje glasova)
- **Flagged** - Komentar je označen od strane korisnika
- **UnFlagged** - Označavanja komentara su uklonjena

### Moderation Actions
- **Pinned** - Komentar je prikačen od strane moderatora (uključuje ko ga je prikačio)
- **UnPinned** - Komentar je otkačen od strane moderatora (uključuje ko ga je otkačio)

### Notification Events
- **CreatedNotifications** - Kreirane su notifikacije za komentar (uključuje broj notifikacija)
- **NotificationCreateFailure** - Kreiranje notifikacija nije uspjelo
- **BadgeAwarded** - Korisniku je dodijeljena značka za komentar (uključuje naziv značke)

### Publishing Events
- **PublishedLive** - Komentar je objavljen uživo pretplatnicima (uključuje broj pretplatnika)

### Integration Events
- **WebhookSynced** - Komentar je sinhronizovan putem webhook-a

### Spam Rule Events
- **SpamRuleMatch** - Komentar se poklopio sa prilagođenim pravilom za spam (uključuje detalje pravila)

### Localization Events
- **LocaleDetectedFromText** - Jezička lokalizacija je automatski detektovana iz teksta komentara (uključuje detektovani jezik i lokal)

## Slučajevi upotrebe zapisa komentara

Zapisi komentara se automatski generišu i pohranjuju sa svakim komentarom. Oni pružaju vrijedne uvide za:

- **Razumijevanje odluka moderiranja** - Precizno vidite zašto je komentar odobren, zadržan na pregledu ili označen kao spam
- **Otklanjanje problema sa odobravanjem/spamom** - Pratite logiku odluka kada komentari ne funkcionišu kako se očekuje
- **Praćenje obrazaca ponašanja korisnika** - Pratite promjene faktora povjerenja i status verifikacije
- **Reviziju akcija moderatora** - Pregledajte koje su radnje moderatori preduzeli nad određenim komentarima
- **Istragu efikasnosti spam filtera** - Vidite koji engine-i otkrivaju spam, a koji ne
- **Rješavanje problema integracija** - Potvrdite sinhronizacije webhook-a i isporuku notifikacija

Ovi zapisi pomažu održati transparentnost u procesu moderiranja i olakšavaju fino podešavanje ponašanja vašeg sistema komentara.