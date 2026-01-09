SSO, odnosno jedinstvena prijava, je skup konvencija koje vam omogućavaju da vi ili vaši korisnici koriste FastComments bez potrebe za kreiranjem dodatnog naloga.

Ako ne dozvoljavate anonimno komentarisanje, nalog je obavezan za komentarisanje sa FastComments. Olakšavamo proces registracije — korisnik samo ostavi svoj e-mail kada komentariše. Ipak, razumijemo da i to predstavlja dodatnu barijeru koju neke stranice žele izbjeći.

Tu barijeru možemo smanjiti tako što ćemo imati samo jedan tok prijave za cijelu vašu stranicu.

### How do I get it?
Svi tipovi naloga trenutno imaju pristup SSO-u. Međutim, maksimalan broj SSO korisnika varira u zavisnosti od vašeg paketa. Kao i za druge funkcije, Pro planovi i viši obezbjeđuju direktnu razvojnu podršku.

Uporedit ćemo opcije, pa zatim ćemo detaljno objasniti svaku.

### User and Comment Migrations

Kada migrirate sa platforme koja koristi SSO, poput Disqus-a, već ćete imati korisnike i njihove komentare.

Komentari se uvoze kao dio vaše migracije, bilo putem API-ja, našeg Import UI-a, ili podrške za korisnike. Import UI je preporučljiv ako podržava platformu iz koje migrirate, jer uključuje rukovanje greškama, ekstrakciju i upload avatara i medija, i sistem za nadzor batch poslova.

Korisnici se dodaju automatski kada po prvi put pregledaju threadove komentara. Alternativno, mogu biti prethodno dodani putem API-ja, ali taj rad nema mnogo prednosti.

Ako su komentari uvezeni, a SSO korisnici nisu dodani ručno putem API-ja, komentari će se automatski migrirati na korisnički nalog prvi put kada se on kreira kada korisnik pregleda bilo koji thread komentara. Tada će moći upravljati, uređivati i brisati komentare koje su prvobitno napisali.

Automatska migracija se vrši putem e-maila ili korisničkog imena. Neke platforme ne daju e-mailove pri eksportu, kao Disqus, pa u tom slučaju prelazimo na korisničko ime.
- Sve dok proslijedite podudarno korisničko ime i e-mail u SSO payloadu, dodaćemo e-mail u pojedinačne objekte komentara kako bi notifikacije i pominjanja funkcionisali.

Ako želite importovati komentare i korisnike odjednom, radite sa podrškom kako biste nakon uvoza korisnika putem API-ja migrirali komentare na odgovarajuće korisničke naloge.

Dakle, da sažmemo najlakši put za migraciju:

1. Importujte komentare.
   1. Avatari i drugi mediji se migriraju automatski ako koristite Import UI u `Manage Data -> Imports`.
2. Podesite Secure ili Simple SSO.
3. Dozvolite da se migracija odvija po korisniku automatski kada se prijave prvi put.
   1. Ovo obično dodaje manje od sekunde na vrijeme učitavanja stranice ako korisnik ima manje od 50k komentara.

### WordPress Users
Ako koristite naš <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress dodatak</a>, nema potrebe za pisanjem koda! Jednostavno otiđite na Admin stranicu dodatka, kliknite SSO Settings, i zatim Enable.

Ovo će vas odvesti do jednostavnog čarobnjaka sa jednim klikom koji će kreirati vaš API key, poslati ga na vašu WordPress instalaciju i uključiti SSO. Konsolidovali smo ovo u jedan klik za vas.

Imajte na umu da ako instalirate dodatak prvi put, moraćete završiti proces podešavanja prije nego što vidite administratorsku stranicu sa dugmetom SSO Settings.

#### WordPress SSO - Moderators

Imajte na umu da trenutno da bi se oznaka "Moderator" prikazala pored vaših moderatora kada komentarišu sa FastComments WordPress dodatkom, oni takođe moraju biti dodani kao Moderator u FastComments dashboardu i imati verifikovan svoj e-mail.

### Custom Integrations

Za Custom integracije postoje dvije opcije.

### Option One - Secure SSO

Sa Secure SSO-om, FastComments zna da je korisnik koji komentariše, glasa i čita komentare stvarni korisnik na vašoj stranici.

Sve dok kreirate validan payload, korisnik će uvijek imati besprekorno iskustvo komentarisanja.

Sa Secure SSO, SSO payload se kreira **server-side** koristeći HMAC authentication i zatim se prosljeđuje widgetu na **client**-u.

Sa Secure SSO, korisnički nalog je **potpuno odvojen** od ostatka FastComments baze korisnika. To znači da ako imamo dva partnera Company A i Company B, svaki od njih može imati SSO korisnika sa korisničkim imenom "Bob".

#### Requirements
- Osnovno znanje o backend razvoju.
- Osnovno znanje o rukovanju tajnim API ključevima.
- Osnovno znanje o razvoju API-ja ili server-side renderovanju.

#### Pros
- Sigurno.
- Besprekorno iskustvo komentarisanja.

#### Cons
- Zahtijeva backend razvoj.

#### Updating User Data

Sa Secure SSO-om, svaki put kada proslijedite sso user payload, mi ćemo ažurirati njihov korisnički nalog sa najnovijim informacijama. Na primjer, ako korisnik ima korisničko ime `X`, i vi proslijedite `Y` u SSO payloadu, njihovo korisničko ime će postati `Y`.

Ako želite ukloniti vrijednosti koristeći ovaj pristup, postavite ih na `null` (ne `undefined`).

#### Secure SSO API

Također pružamo API za interakciju sa SSO korisnicima. Pogledajte [dokumentacija](/guide-api.html#sso-user-structure).

Imajte na umu da pri korištenju Secure SSO-a, korisnici se automatski kreiraju u pozadini pri učitavanju stranice. Ne morate masovno importovati svoje korisnike.

### Option Two - Simple SSO

Alternativa Secure SSO-u je jednostavno prosljeđivanje informacija o korisniku widgetu za komentarisanje.

Nije obavezno prosljeđivati e-mail kod Simple SSO-a, međutim bez njega njihovi komentari će se prikazivati kao "Unverified".

<sup>NAPOMENA!</sup> Od početka 2022. korisnička imena kod Simple SSO-a ne moraju biti jedinstvena preko cijelog FastComments.com.

Idealno, Simple SSO bi se trebao birati samo kada razvijate na platformi koja ne pruža pristup backendu.

#### Requirements
- Osnovno znanje o client-side razvoju.
- Potrebno je znati bar e-mail korisnika.

#### Pros
- Jednostavno.
- Sva aktivnost se i dalje verifikuje.
- Korisnik nikada ne unosi svoje korisničko ime ili e-mail.

#### Cons
- Manje sigurno nego Secure SSO jer klijentski payload može biti konstruisan da postane bilo koji korisnik.

#### Simple SSO API

Korisnici koji su automatski kreirani putem Simple SSO toka se čuvaju kao `SSOUser` objekti. Mogu im se pristupiti i upravljati putem `SSOUser` API-ja. Pogledajte [dokumentacija](/guide-api.html#sso-user-structure).