FastComments automatski prati detaljne događaje za svaki komentar kako bi pružio transparentnost u odlukama moderacije i sistemskim akcijama. Ovi dnevnici pomažu vam da razumete zašto je komentar odobren, označen kao spam ili mu je promenjen status.

## Pristupanje dnevnicima komentara

Da biste pregledali dnevnike za određeni komentar:

1. Idite na stranicu **Moderate Comments** u vašem FastComments kontrolnom panelu  
2. Pronađite komentar koji želite da pregledate  
3. Kliknite na dugme **View Logs** (ikona sata) u traci akcija komentara  
4. Pojaviće se dijalog koji prikazuje kompletnu istoriju događaja za taj komentar  

Svaki unos u dnevniku prikazuje:
- **When** - Vremenska oznaka događaja  
- **Who** - Korisnik ili sistem koji je pokrenuo događaj (kada je primenljivo)  
- **What** - Tip akcije ili događaja  
- **Details** - Dodatni kontekst kao što su vrednosti pre/posle, nazivi motora ili povezani podaci  

## Događaji dnevnika komentara

Svaki komentar održava dnevnik događaja koji se dešavaju tokom njegovog životnog ciklusa. Ispod su tipovi događaja koji se prate:

### Anonimizacija događaja
- **Anonymized** - Sadržaj komentara je očišćen i korisnik označen kao obrisan  
- **RestoredFromAnonymized** - Komentar je vraćen iz anonimizovanog stanja  

### Događaji odobrenja
- **ApprovedDueToPastComment** - Komentar odobren jer je korisnik prethodno odobravao komentare (uključuje referencu na prethodni komentar)  
- **ApprovedIsAdmin** - Komentar odobren jer je korisnik administrator  
- **NotApprovedRequiresApproval** - Komentar zahteva ručno odobrenje  
- **NotApprovedLowTrustFactor** - Komentar nije odobren zbog niskog faktora poverenja korisnika (uključuje vrednost faktora poverenja)  

### Događaji odobrenja komentara na profilu
Ovi događaji se primenjuju specifično na komentare na korisničkim profilima:

- **ApprovedProfileAutoApproveAll** - Komentar na profilu automatski odobren jer je vlasnik profila omogućio automatsko odobravanje za sve komentare  
- **ApprovedProfileTrusted** - Komentar na profilu odobren jer je komentator pouzdan (uključuje referencu na komentar koji je uspostavio poverenje)  
- **NotApprovedProfileManualApproveAll** - Komentar na profilu zahteva ručno odobrenje jer je vlasnik profila omogućio ručno odobravanje  
- **NotApprovedProfileNotTrusted** - Komentar na profilu nije odobren jer komentator nije pouzdan  
- **NotApprovedProfileNewUser** - Komentar na profilu nije odobren jer je komentator novi korisnik  

### Događaji otkrivanja spama
- **IsSpam** - Komentar označen kao spam od strane motora za detekciju (uključuje koji je motor donio odluku)  
- **IsSpamDueToBadWords** - Komentar označen kao spam zbog filtera za psovke  
- **IsSpamFromLLM** - Komentar označen kao spam od strane AI/LLM motora (uključuje ime motora, odgovor i broj tokena)  
- **IsSpamRepeatComment** - Komentar označen kao spam zbog ponavljanja (uključuje koji je motor otkrio to)  
- **NotSpamIsOnlyImage** - Komentar nije označen kao spam jer sadrži samo slike  
- **NotSpamIsOnlyReacts** - Komentar nije označen kao spam jer sadrži samo reakcije  
- **NotSpamNoLinkOrMention** - Komentar nije označen kao spam zbog nedostatka sumnjivih linkova ili spominjanja  
- **NotSpamPerfectTrustFactor** - Komentar nije označen kao spam zbog visokog poverenja korisnika  
- **NotSpamTooShort** - Komentar nije označen kao spam jer je prekratak za analizu  
- **NotSpamSkipped** - Provera spama je preskočena  
- **NotSpamFromEngine** - Komentar je odredjen da nije spam od strane motora za detekciju (uključuje ime motora i faktor poverenja)  

### Događaji loših reči/profaniteta
- **BadWordsCheckFailed** - Provera filtera za psovke je naišla na grešku  
- **BadWordsFoundBadPhrase** - Filter za psovke je otkrio neprimerenu frazu (uključuje frazu)  
- **BadWordsFoundBadWord** - Filter za psovke je otkrio neprimerenu reč (uključuje reč)  
- **BadWordsNoDefinitionForLocale** - Nema definicija profanosti za jezik komentara (uključuje lokalitet)  

### Događaji verifikacije korisnika
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahteva verifikaciju, ali korisnik nije u verifikovanoj sesiji  
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahteva verifikaciju, ali korisnik još nije verifikovan  
- **InVerifiedSession** - Korisnik koji postavlja komentar je u verifikovanoj sesiji  
- **SentVerificationEmailNoSession** - Email za verifikaciju poslat neverifikovanom korisniku  
- **SentWelcomeEmail** - Email dobrodošlice poslat novom korisniku  

### Događaji poverenja i sigurnosti
- **TrustFactorChanged** - Faktor poverenja korisnika je izmenjen (uključuje vrednosti pre i posle)  
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama zaobiđeno za admin korisnika  
- **TenantSpamFilterDisabled** - Filtriranje spama onemogućeno za celog tenant-a  
- **RepeatCommentCheckIgnored** - Provera ponovljenog komentara je zaobiđena (uključuje razlog)  
- **UserIsAdmin** - Korisnik je identifikovan kao admin  
- **UserIsAdminParentTenant** - Korisnik je identifikovan kao admin roditeljskog tenant-a  
- **UserIsAdminViaSSO** - Korisnik je identifikovan kao admin putem SSO  
- **UserIsMod** - Korisnik je identifikovan kao moderator  

### Promene statusa komentara
Događaji promene statusa uključuju vrednosti pre i posle, plus korisnika koji je izvršio promenu:

- **ExpireStatusChanged** - Status isteka komentara je izmenjen  
- **ReviewStatusChanged** - Status pregleda komentara je promenjen  
- **SpamStatusChanged** - Status spama komentara je ažuriran  
- **ApproveStatusChanged** - Status odobrenja komentara je promenjen  
- **TextChanged** - Sadržaj teksta komentara je izmenjen (uključuje tekst pre i posle)  
- **VotesChanged** - Broj glasova komentara je ažuriran (uključuje detaljan pregled glasova)  
- **Flagged** - Komentar je označen od strane korisnika  
- **UnFlagged** - Oznake komentara su uklonjene  

### Akcije moderacije
- **Pinned** - Komentar je zakačen od strane moderatora (uključuje ko ga je zakačio)  
- **UnPinned** - Komentar je otkačen od strane moderatora (uključuje ko ga je otkačio)  

### Događaji obaveštenja
- **CreatedNotifications** - Obaveštenja su kreirana za komentar (uključuje broj obaveštenja)  
- **NotificationCreateFailure** - Neuspešno kreiranje obaveštenja  
- **BadgeAwarded** - Korisniku je dodeljena značka za komentar (uključuje ime značke)  

### Događaji spominjanja i obaveštenja o odgovorima
Ovi događaji navode osobu koja bi primila email ili obaveštenje. Kada ništa nije poslato, kolona Detalji navodi razlog.

- **MentionEmailSent** - Korisniku spomenutom u komentaru je poslat email  
- **MentionEmailSkipped** - Spomenuti korisnik nije dobio email (uključuje razlog)  
- **MentionHeldForApproval** - Email spominjanja čeka dok se komentar ne odobri  
- **MentionNotificationCreated** - Spomenuti korisnik je dobio obaveštenje u aplikaciji  
- **MentionNotificationSkipped** - Spomenuti korisnik nije dobio obaveštenje u aplikaciji (uključuje razlog)  
- **ReplyEmailSent** - Autor komentara na koji se odgovara je obavešten emailom o ovom odgovoru  
- **ReplyEmailSkipped** - Autor komentara na koji se odgovara nije obavešten emailom (uključuje razlog)  
- **ReplyNotificationSkipped** - Autor komentara na koji se odgovara nije dobio obaveštenje u aplikaciji (uključuje razlog)  

Razlozi prikazani kada email ili obaveštenje nisu poslati:
- Korisnik više ne postoji ili nema email adresu  
- Korisnik je isključio email obaveštenja ili obaveštenja za tu nit  
- Jedan od dva korisnika je blokirao drugog  
- Korisnici nisu u istim SSO grupama  
- Email adresa korisnika je na listi za suzbijanje nakon odbijanja ili pritužbe na spam (pogledajte [Email Suppression Management](/guide-notifications.html#email-suppression-management))  
- Email adresa korisnika je na example.com, što ne može primati email  
- Komentar je označen kao spam, obrisan ili nije odobren u roku od 7 dana  
- Komentar na koji se odgovara ostavljen je anonimno  
- Korisnik je odgovorio na svoj komentar  
- Korisnik je spomenut u odgovoru, pa je dobio email spominjanja umesto emaila odgovora  
- Korisnik je već imao obaveštenje o odgovoru za komentar  
- Slanje je neuspešno 5 puta  

Ako isporuka ne uspe ili dostigne limit slanja, email se stavlja u red za ponovni pokušaj i unos u dnevniku to navodi.

### Događaji objavljivanja
- **PublishedLive** - Komentar je objavljen živim pretplatnicima (uključuje broj pretplatnika)  

### Događaji integracije
- **WebhookSynced** - Komentar je sinhronizovan putem webhook-a  

### Događaji pravila spama
- **SpamRuleMatch** - Komentar je odgovarao prilagođenom pravilu spama (uključuje detalje pravila)  

### Događaji lokalizacije
- **LocaleDetectedFromText** - Jezički lokalitet je automatski detektovan iz teksta komentara (uključuje detektovani jezik i lokalitet)  

## Upotrebe dnevnika komentara

Dnevnici komentara se automatski generišu i čuvaju uz svaki komentar. Oni pružaju dragocene uvide za:
- **Understanding moderation decisions** - Vidite tačno zašto je komentar odobren, zadržan za pregled ili označen kao spam  
- **Debugging approval/spam issues** - Pratite logiku odluke kada komentari ne funkcionišu kako se očekuje  
- **Tracking user behavior patterns** - Pratite promene faktora poverenja i status verifikacije  
- **Auditing moderator actions** - Pregledajte koje su akcije moderatora preduzete na određenim komentarima  
- **Investigating spam filter effectiveness** - Vidite koji motori za detekciju hvataju spam, a koji ne  
- **Troubleshooting integrations** - Verifikujte sinhronizacije webhook-a i isporuku obaveštenja  

Ovi dnevnici pomažu u održavanju transparentnosti u procesu moderacije i pomažu u finom podešavanju ponašanja vašeg sistema za komentare.