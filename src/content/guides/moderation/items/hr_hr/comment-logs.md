FastComments automatski prati detaljne događaje za svaki komentar kako bi pružio transparentnost u odluke moderacije i radnje sustava. Ovi zapisi pomažu vam razumjeti zašto je komentar odobren, označen kao spam ili mu je promijenjen status.

## Pristup zapisima komentara

1. Idite na stranicu **Moderate Comments** u vašem FastComments nadzornom panelu  
2. Pronađite komentar koji želite pregledati  
3. Kliknite na gumb **View Logs** (ikona sata) u traci radnji komentara  
4. Pojavit će se dijalog koji prikazuje cjelovitu povijest događaja za taj komentar  

Svaki unos u zapisu prikazuje:
- **When** - Vremenska oznaka događaja  
- **Who** - Korisnik ili sustav koji je pokrenuo događaj (ako je primjenjivo)  
- **What** - Vrsta radnje ili događaja  
- **Details** - Dodatni kontekst poput vrijednosti prije/poslije, naziva motora ili povezanih podataka  

## Događaji zapisa komentara

Svaki komentar održava zapis događaja koji se događaju tijekom njegovog životnog ciklusa. Ispod su vrste događaja koji se prate:

### Događaji anonimizacije
- **Anonymized** - Sadržaj komentara je očišćen i korisnik označen kao izbrisan  
- **RestoredFromAnonymized** - Komentar je obnovljen iz anonimiziranog stanja  

### Događaji odobrenja
- **ApprovedDueToPastComment** - Komentar odobren jer je korisnik prethodno odobrio komentare (uključuje referencu na prethodni komentar)  
- **ApprovedIsAdmin** - Komentar odobren jer je korisnik administrator  
- **NotApprovedRequiresApproval** - Komentar zahtijeva ručno odobrenje  
- **NotApprovedLowTrustFactor** - Komentar nije odobren zbog niskog faktora povjerenja korisnika (uključuje vrijednost faktora povjerenja)  

### Događaji odobrenja komentara na profilu
Ovi događaji primjenjuju se specifično na komentare na korisničkim profilima:
- **ApprovedProfileAutoApproveAll** - Komentar na profilu automatski odobren jer je vlasnik profila omogućio automatsko odobravanje za sve komentare  
- **ApprovedProfileTrusted** - Komentar na profilu odobren jer je komentator pouzdan (uključuje referencu na komentar koji je uspostavio povjerenje)  
- **NotApprovedProfileManualApproveAll** - Komentar na profilu zahtijeva ručno odobrenje jer je vlasnik profila omogućio ručno odobravanje  
- **NotApprovedProfileNotTrusted** - Komentar na profilu nije odobren jer komentator nije pouzdan  
- **NotApprovedProfileNewUser** - Komentar na profilu nije odobren jer je komentator novi korisnik  

### Događaji otkrivanja spama
- **IsSpam** - Komentar označen kao spam od strane motora za otkrivanje (uključuje koji je motor donio odluku)  
- **IsSpamDueToBadWords** - Komentar označen kao spam zbog filtera za psovke  
- **IsSpamFromLLM** - Komentar označen kao spam od AI/LLM motora (uključuje ime motora, odgovor i broj tokena)  
- **IsSpamRepeatComment** - Komentar označen kao spam zbog ponavljanja (uključuje koji je motor otkrio)  
- **NotSpamIsOnlyImage** - Komentar nije označen kao spam jer sadrži samo slike  
- **NotSpamIsOnlyReacts** - Komentar nije označen kao spam jer sadrži samo reakcije  
- **NotSpamNoLinkOrMention** - Komentar nije označen kao spam zbog nedostatka sumnjivih veza ili spominjanja  
- **NotSpamPerfectTrustFactor** - Komentar nije označen kao spam zbog visokog povjerenja korisnika  
- **NotSpamTooShort** - Komentar nije označen kao spam jer je prekratak za analizu  
- **NotSpamSkipped** - Provjera spama je preskočena  
- **NotSpamFromEngine** - Komentar je proglašen ne-spamom od strane motora za otkrivanje (uključuje ime motora i faktor povjerenja)  

### Događaji loših riječi/profaniteta
- **BadWordsCheckFailed** - Provjera filtera za psovke naišla je na grešku  
- **BadWordsFoundBadPhrase** - Filter za psovke otkrio je neprimjerenu frazu (uključuje frazu)  
- **BadWordsFoundBadWord** - Filter za psovke otkrio je neprimjerenu riječ (uključuje riječ)  
- **BadWordsNoDefinitionForLocale** - Nema definicija profanosti za jezik komentara (uključuje lokalitet)  

### Događaji provjere korisnika
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahtijeva verifikaciju, ali korisnik nije u verificiranoj sesiji  
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahtijeva verifikaciju, ali korisnik još nije verificiran  
- **InVerifiedSession** - Korisnik koji objavljuje komentar je u verificiranoj sesiji  
- **SentVerificationEmailNoSession** - Email za verifikaciju poslan neverificiranom korisniku  
- **SentWelcomeEmail** - Email dobrodošlice poslan novom korisniku  

### Događaji povjerenja i sigurnosti
- **TrustFactorChanged** - Faktor povjerenja korisnika je izmijenjen (uključuje vrijednosti prije i poslije)  
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama zaobiđeno za admin korisnika  
- **TenantSpamFilterDisabled** - Filtriranje spama onemogućeno za cijelog najmodavca  
- **RepeatCommentCheckIgnored** - Provjera ponavljanja komentara je zanemarena (uključuje razlog)  
- **UserIsAdmin** - Korisnik identificiran kao admin  
- **UserIsAdminParentTenant** - Korisnik identificiran kao admin nadređenog najmodavca  
- **UserIsAdminViaSSO** - Korisnik identificiran kao admin putem SSO  
- **UserIsMod** - Korisnik identificiran kao moderator  

### Promjene statusa komentara
Događaji promjene statusa uključuju vrijednosti prije i poslije, plus korisnika koji je izvršio promjenu:
- **ExpireStatusChanged** - Status isteka komentara je izmijenjen  
- **ReviewStatusChanged** - Status pregleda komentara je promijenjen  
- **SpamStatusChanged** - Status spama komentara je ažuriran  
- **ApproveStatusChanged** - Status odobrenja komentara je promijenjen  
- **TextChanged** - Sadržaj teksta komentara je uređen (uključuje tekst prije i poslije)  
- **VotesChanged** - Broj glasova komentara je ažuriran (uključuje detaljan prikaz glasova)  
- **Flagged** - Komentar je označen od strane korisnika  
- **UnFlagged** - Oznake komentara su uklonjene  

### Akcije moderacije
- **Pinned** - Komentar je zakačen od strane moderatora (uključuje tko ga je zakačio)  
- **UnPinned** - Komentar je odkačen od strane moderatora (uključuje tko ga je odkačio)  

### Događaji obavijesti
- **CreatedNotifications** - Obavijesti su kreirane za komentar (uključuje broj obavijesti)  
- **NotificationCreateFailure** - Neuspjelo kreiranje obavijesti  
- **BadgeAwarded** - Korisniku je dodijeljena značka za komentar (uključuje naziv značke)  

### Događaji spominjanja i obavijesti o odgovorima
Ovi događaji navode osobu koja bi primila email ili obavijest. Kada ništa nije poslano, stupac Detalji navodi razlog.
- **MentionEmailSent** - Korisniku spomenutom u komentaru poslan je email  
- **MentionEmailSkipped** - Spomenuti korisnik nije primio email (uključuje razlog)  
- **MentionHeldForApproval** - Email spominjanja čeka dok se komentar ne odobri  
- **MentionNotificationCreated** - Spomenuti korisnik je dobio obavijest u aplikaciji  
- **MentionNotificationSkipped** - Spomenuti korisnik nije dobio obavijest u aplikaciji (uključuje razlog)  
- **ReplyEmailSent** - Autor komentara na koji se odgovara je obaviješten emailom o ovom odgovoru  
- **ReplyEmailSkipped** - Autor komentara na koji se odgovara nije obaviješten emailom (uključuje razlog)  
- **ReplyNotificationSkipped** - Autor komentara na koji se odgovara nije dobio obavijest u aplikaciji (uključuje razlog)  

Razlozi prikazani kada email ili obavijest nisu poslani:
- Korisnik više ne postoji ili nema email adresu  
- Korisnik je isključio email obavijesti ili isključio obavijesti za tu nit  
- Jedan od dva korisnika je blokirao drugog  
- Korisnici nisu u istim SSO grupama  
- Email adresa korisnika je na listi suzbijanja nakon odbijanja ili pritužbe na spam (pogledajte [Email Suppression Management](/guide-notifications.html#email-suppression-management))  
- Email adresa korisnika je na example.com, što ne može primati emailove  
- Komentar je označen kao spam, izbrisan ili nije odobren u roku od 7 dana  
- Komentar na koji se odgovara ostavljen je anonimno  
- Korisnik je odgovorio na vlastiti komentar  
- Korisnik je spomenut u odgovoru, pa je dobio email spominjanja umjesto emaila odgovora  
- Korisnik je već imao obavijest o odgovoru za komentar  
- Slanje je neuspjelo 5 puta  

Ako isporuka ne uspije ili dosegne limit slanja, email se stavlja u red za ponovni pokušaj i zapis u logu to navodi.

### Događaji objavljivanja
- **PublishedLive** - Komentar je objavljen živim pretplatnicima (uključuje broj pretplatnika)  

### Događaji integracije
- **WebhookSynced** - Komentar je sinkroniziran putem webhooka  

### Događaji pravila spama
- **SpamRuleMatch** - Komentar je podudaran s prilagođenim pravilom spama (uključuje detalje pravila)  

### Događaji lokalizacije
- **LocaleDetectedFromText** - Jezični lokalitet je automatski otkriven iz teksta komentara (uključuje otkriveni jezik i lokalitet)  

## Primjeri upotrebe zapisa komentara

Zapisi komentara automatski se generiraju i pohranjuju uz svaki komentar. Oni pružaju vrijedne uvide za:
- **Understanding moderation decisions** - Točno vidjeti zašto je komentar odobren, zadržan za pregled ili označen kao spam  
- **Debugging approval/spam issues** - Pratiti logiku odluke kada komentari ne rade kako se očekuje  
- **Tracking user behavior patterns** - Pratiti promjene faktora povjerenja i status verifikacije  
- **Auditing moderator actions** - Pregledati koje su akcije moderatora poduzete na određenim komentarima  
- **Investigating spam filter effectiveness** - Vidjeti koji motori otkrivanja hvataju spam, a koji ne  
- **Troubleshooting integrations** - Provjeriti sinkronizacije webhooka i isporuku obavijesti  

Ovi zapisi pomažu održati transparentnost u procesu moderacije i pomažu u finom podešavanju ponašanja vašeg sustava komentara.