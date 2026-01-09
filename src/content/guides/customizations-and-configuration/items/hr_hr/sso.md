SSO, odnosno prijava jednom (single-sign-on), je skup konvencija koje omogućuju vama ili vašim korisnicima korištenje FastComments bez potrebe za stvaranjem dodatnog računa.

Pod pretpostavkom da ne dopuštate anonimno komentiranje, račun je potreban za komentiranje putem FastComments. Pojednostavljujemo proces prijave — korisnik jednostavno ostavi svoj email kad komentira.
Međutim, razumijemo da i to može predstavljati dodatnu prepreku koju neki sajtovi žele izbjeći.

Tu trenutačno možemo smanjiti tu zapreku tako da postoji samo jedan tok prijave za cijeli vaš sajt.

### Kako dobijem pristup?
Sve vrste računa trenutno dobivaju pristup SSO-u. Međutim, maksimalan broj SSO korisnika razlikovat će se ovisno o vašem paketu. Kao i kod drugih značajki, Pro planovi i viši nude izravnu razvojnu podršku.

Usporedimo opcije, a zatim ćemo ući u detalje svake od njih.

### Migracije korisnika i komentara

Prilikom migracije s platforme koja koristi SSO, kao što je Disqus, već ćete imati korisnike i njihove komentare.

Komentari se uvoze kao dio vaše migracije, bilo putem API-ja, našeg Import UI-a ili korisničke podrške. Import UI je poželjan ako podržava platformu s koje migrirate, jer uključuje obradu pogrešaka, izdvajanje i učitavanje avatara i medija te sustav za praćenje batch poslova.

Korisnici se dodaju automatski pri prvom otvaranju niti komentara. Alternativno, mogu se prethodno dodati putem API-ja, ali taj postupak nema mnogo prednosti.

Ako su komentari uvezeni, a SSO korisnici nisu ručno dodani putem API-ja, komentari će se automatski premjestiti na račun korisnika prvi put kada se on kreira pri pregledavanju bilo koje niti komentara. Tada će moći uređivati i brisati komentare koje su izvorno napisali.

Automatska migracija vrši se putem emaila ili username-a. Neke platforme ne daju emailove pri eksportu, poput Disqus-a, pa u tom slučaju koristimo username.
- Sve dok pošaljete odgovarajući username i email u SSO payloadu, mi ćemo dodati email na pojedinačne objekte komentara kako bi obavijesti i spominjanja radili.

Ako želite uvesti komentare i korisnike odjednom, surađujte s podrškom kako bi komentari bili migrirani na odgovarajuće korisničke račune nakon što su korisnici uvezeni putem API-ja.

Dakle, za sažetak, najlakši put za migraciju je:

1. Uvezite komentare.
   1. Avatari i drugi mediji se migriraju automatski ako koristite Import UI u `Manage Data -> Imports`.
2. Postavite Secure ili Simple SSO.
3. Dopustite da migracija nastupi po korisniku automatski kad se prvi put prijave.
   1. To obično dodaje manje od sekunde na vrijeme učitavanja stranice ako korisnik ima manje od 50k komentara.

### Korisnici WordPress-a
Ako koristite naš <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress dodatak</a>, nema potrebe pisati kod! Jednostavno idite na administratorsku stranicu dodatka, kliknite SSO Settings, a zatim Enable.

To će vas odvesti na čarobnjak s jednom tipkom koji će kreirati vaš API ključ, poslati ga na vašu WordPress instalaciju i uključiti SSO. Ovo smo objedili u jedan klik za vas.

Imajte na umu da ako instalirate dodatak prvi put, morat ćete dovršiti proces postavljanja prije nego što vidite administratorsku stranicu s gumbom SSO Settings.

#### WordPress SSO - Moderatori

Imajte na umu da se trenutno za prikaz značke "Moderator" pored vaših moderatora kada komentiraju putem FastComments WordPress dodatka,
moraju također dodati kao Moderator u FastComments nadzornoj ploči i imati verificiran email.

### Prilagođene integracije

Za prilagođene integracije postoje dvije opcije.

### Opcija jedna - Secure SSO

Sa Secure SSO-om, FastComments zna da je korisnik koji komentira, glasa i čita komentare stvarni korisnik na vašem sajtu.

Sve dok kreirate valjan payload, korisnik će uvijek imati besprijekorno iskustvo komentiranja.

Sa Secure SSO-om, SSO payload se kreira **server-side** koristeći HMAC authentication i zatim se prosljeđuje widgetu na **client** strani.

Sa Secure SSO-om, korisnički račun je **potpuno odvojen** od ostatka FastComments baze korisnika. To znači da ako imamo dva partnera
Company A i Company B, svaki može imati SSO korisnika s username-om "Bob".

#### Zahtjevi
- Osnovno znanje o backend razvoju.
- Osnovno znanje o radu s tajnim API ključevima.
- Osnovno znanje o razvoju API-ja ili server-side renderiranju.

#### Prednosti
- Sigurno.
- Besprijekorno iskustvo komentiranja.

#### Nedostaci
- Zahtijeva razvoj na backend-u.

#### Ažuriranje podataka korisnika

Sa Secure SSO-om, svaki put kada pošaljete sso user payload, mi ćemo ažurirati njihov korisnički račun s najnovijim informacijama. Na primjer, ako
korisnik ima username `X`, i vi pošaljete `Y` u SSO payloadu, njihov username će postati `Y`.

Ako želite ukloniti vrijednosti koristeći ovaj pristup, postavite ih na `null` (ne `undefined`).

#### Secure SSO API

Također pružamo API za interakciju sa SSO korisnicima. Pogledajte [the docs](/guide-api.html#sso-user-structure).

Imajte na umu da se pri korištenju Secure SSO-a korisnici automatski kreiraju u pozadini pri učitavanju stranice. Ne morate masovno uvoziti svoje korisnike.

### Opcija dva - Simple SSO

Alternativa Secure SSO-u je jednostavno proslijediti korisničke informacije widgetu za komentiranje.

Nije obavezno proslijediti email s Simple SSO-om, međutim bez njega njihovi komentari će se prikazivati kao "Unverified".

<sup>Note!</sup> Od početka 2022. username-i kod Simple SSO-a ne moraju biti jedinstveni preko cijelog FastComments.com.

Idealno, Simple SSO bi se trebao odabrati samo pri razvoju na platformi koja ne omogućuje pristup backend-u.

#### Zahtjevi
- Osnovno znanje o razvoju na strani klijenta.
- Morate poznavati barem email korisnika.

#### Prednosti
- Jednostavno.
- Sva aktivnost se i dalje verificira.
- Korisnik nikad ne unosi svoj username ili email.

#### Nedostaci
- Manje sigurno od Secure SSO-a jer payload na strani klijenta može biti konstruiran da postane bilo koji korisnik.

#### Simple SSO API

Korisnici automatski kreirani putem Simple SSO toka pohranjuju se kao `SSOUser` objekti. Do njih se može pristupiti i upravljati putem `SSOUser` API-ja. Pogledajte [the docs](/guide-api.html#sso-user-structure).