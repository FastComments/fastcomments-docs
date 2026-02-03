FastComments automatski prati detaljne događaje za svaki komentar kako bi osigurao transparentnost odluka o moderiranju i akcija sustava. Ti zapisi pomažu vam razumjeti zašto je komentar odobren, označen kao spam ili mu je promijenjen status.

## Accessing Comment Logs

Za pregled zapisnika za određeni komentar:

1. Idite na stranicu **Moderate Comments** u vašoj FastComments nadzornoj ploči
2. Pronađite komentar koji želite pregledati
3. Kliknite gumb **View Logs** (ikona sata) u traci radnji komentara
4. Pojavit će se dijalog koji prikazuje kompletnu povijest događaja za taj komentar

Svaki unos u zapisu prikazuje:
- **When** - Vremenska oznaka događaja
- **Who** - Korisnik ili sustav koji je pokrenuo događaj (kad je primjenjivo)
- **What** - Vrsta akcije ili događaja
- **Details** - Dodatni kontekst poput vrijednosti prije/poslije, naziva motora ili povezanih podataka

## Comment Log Events

Svaki komentar održava zapis događaja koji se javljaju tijekom njegovog životnog ciklusa. Ispod su vrste događaja koje se prate:

### Anonymization Events
- **Anonymized** - Sadržaj komentara je obrisan i korisnik označen kao izbrisan
- **RestoredFromAnonymized** - Komentar je vraćen iz anonimiziranog stanja

### Approval Events
- **ApprovedDueToPastComment** - Komentar odobren jer korisnik ima prethodno odobrene komentare (uključuje referencu na prethodni komentar)
- **ApprovedIsAdmin** - Komentar odobren jer je korisnik administrator
- **NotApprovedRequiresApproval** - Komentar zahtijeva ručno odobrenje
- **NotApprovedLowTrustFactor** - Komentar nije odobren zbog niske vrijednosti povjerenja korisnika (uključuje vrijednost povjerenja)

### Profile Comment Approval Events

Ovi događaji se primjenjuju posebno na komentare na korisničkim profilima:

- **ApprovedProfileAutoApproveAll** - Komentar na profilu automatski odobren jer je vlasnik profila omogućio automatsko odobravanje svih komentara
- **ApprovedProfileTrusted** - Komentar na profilu odobren jer je komentator pouzdan (uključuje referencu na komentar koji je uspostavio povjerenje)
- **NotApprovedProfileManualApproveAll** - Komentar na profilu zahtijeva ručno odobrenje jer je vlasnik profila omogućio ručno odobravanje
- **NotApprovedProfileNotTrusted** - Komentar na profilu nije odobren jer komentator nije pouzdan
- **NotApprovedProfileNewUser** - Komentar na profilu nije odobren jer je komentator novi korisnik

### Spam Detection Events
- **IsSpam** - Komentar označen kao spam od strane motora za detekciju (uključuje koji je motor donio odluku)
- **IsSpamDueToBadWords** - Komentar označen kao spam zbog filtra nepristojnih riječi
- **IsSpamFromLLM** - Komentar označen kao spam od strane AI/LLM motora (uključuje naziv motora, odgovor i broj tokena)
- **IsSpamRepeatComment** - Komentar označen kao spam zbog ponavljanja (uključuje koji je motor to detektirao)
- **NotSpamIsOnlyImage** - Komentar nije označen kao spam jer sadrži samo slike
- **NotSpamIsOnlyReacts** - Komentar nije označen kao spam jer sadrži samo reakcije
- **NotSpamNoLinkOrMention** - Komentar nije označen kao spam zbog izostanka sumnjivih poveznica ili spominjanja
- **NotSpamPerfectTrustFactor** - Komentar nije označen kao spam zbog visokog povjerenja korisnika
- **NotSpamTooShort** - Komentar nije označen kao spam jer je premalen za analizu
- **NotSpamSkipped** - Provjera spama je preskočena
- **NotSpamFromEngine** - Komentar je od detekcijskog motora označen kao ne-spam (uključuje naziv motora i faktor povjerenja)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Provjera filtra nepristojnih riječi naišla je na grešku
- **BadWordsFoundBadPhrase** - Filtriranje nepristojnih riječi pronašlo je neprimjerenu frazu (uključuje frazu)
- **BadWordsFoundBadWord** - Filtriranje nepristojnih riječi pronašlo je neprimjerenu riječ (uključuje riječ)
- **BadWordsNoDefinitionForLocale** - Nema definicija nepristojnih riječi dostupnih za jezik komentara (uključuje lokalitet)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahtijeva verifikaciju, ali korisnik nije u verificiranoj sesiji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahtijeva verifikaciju, ali korisnik još nije verificiran
- **InVerifiedSession** - Korisnik koji objavljuje komentar je u verificiranoj sesiji
- **SentVerificationEmailNoSession** - Verifikacijski e-mail poslan neverificiranom korisniku
- **SentWelcomeEmail** - Poslan e-mail dobrodošlice novom korisniku

### Trust and Security Events
- **TrustFactorChanged** - Faktor povjerenja korisnika je izmijenjen (uključuje vrijednosti prije i poslije)
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama zaobiđeno za administratorskog korisnika
- **TenantSpamFilterDisabled** - Filtriranje spama onemogućeno za cijeli tenant
- **RepeatCommentCheckIgnored** - Provjera ponovljenih komentara je zaobiđena (uključuje razlog)
- **UserIsAdmin** - Korisnik identificiran kao administrator
- **UserIsAdminParentTenant** - Korisnik identificiran kao administrator roditeljskog tenanta
- **UserIsAdminViaSSO** - Korisnik identificiran kao administrator putem SSO
- **UserIsMod** - Korisnik identificiran kao moderator

### Comment Status Changes

Događaji promjene statusa uključuju vrijednosti prije i poslije, te korisnika koji je izvršio promjenu:

- **ExpireStatusChanged** - Status isteka komentara je izmijenjen
- **ReviewStatusChanged** - Status recenzije komentara je promijenjen
- **SpamStatusChanged** - Spam status komentara je ažuriran
- **ApproveStatusChanged** - Status odobrenja komentara je promijenjen
- **TextChanged** - Tekst komentara je uređen (uključuje tekst prije i poslije)
- **VotesChanged** - Broj glasova komentara je ažuriran (uključuje detaljnu raščlambu glasova)
- **Flagged** - Komentar je označen od strane korisnika
- **UnFlagged** - Označavanja komentara su uklonjena

### Moderation Actions
- **Pinned** - Komentar je prikvačen od strane moderatora (uključuje tko ga je prikvačio)
- **UnPinned** - Komentar je otkvačen od strane moderatora (uključuje tko ga je otkvačio)

### Notification Events
- **CreatedNotifications** - Za komentar su kreirane obavijesti (uključuje broj obavijesti)
- **NotificationCreateFailure** - Neuspjelo stvaranje obavijesti
- **BadgeAwarded** - Korisniku je dodijeljena značka za komentar (uključuje naziv značke)

### Publishing Events
- **PublishedLive** - Komentar je objavljen uživo pretplatnicima (uključuje broj pretplatnika)

### Integration Events
- **WebhookSynced** - Komentar je sinkroniziran putem webhooka

### Spam Rule Events
- **SpamRuleMatch** - Komentar se poklapao s prilagođenim pravilom protiv spama (uključuje detalje pravila)

### Localization Events
- **LocaleDetectedFromText** - Jezični lokalitet je automatski otkriven iz teksta komentara (uključuje otkriveni jezik i lokalitet)

## Use Cases for Comment Logs

Zapisnici komentara se automatski generiraju i pohranjuju uz svaki komentar. Oni pružaju vrijedne uvide za:

- **Understanding moderation decisions** - Vidjeti točno zašto je komentar odobren, zadržan na pregledu ili označen kao spam
- **Debugging approval/spam issues** - Pratiti logiku odluka kada komentari ne ponašaju očekivano
- **Tracking user behavior patterns** - Pratiti promjene faktora povjerenja i status verifikacije
- **Auditing moderator actions** - Pregledati koje su akcije moderatori poduzeli nad konkretnim komentarima
- **Investigating spam filter effectiveness** - Vidjeti koji motori za detekciju pronalaze spam, a koji ne
- **Troubleshooting integrations** - Provjeriti sinkronizacije webhooka i isporuku obavijesti

Ovi zapisi pomažu održati transparentnost u procesu moderiranja i pomažu u finoj prilagodbi ponašanja vašeg sustava komentara.