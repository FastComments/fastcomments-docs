SSO, ili single-sign-on, je skup konvencija koje vam ili vašim korisnicima omogućavaju da koristite FastComments bez potrebe za kreiranjem novog naloga.

Pod pretpostavkom da ne dozvoljavate anonimno komentiranje, nalog je potreban za komentiranje putem FastComments. Proces registracije činimo vrlo jednostavnim - korisnik samo ostavlja svoj email prilikom komentiranja.
Ipak, razumemo da i to predstavlja dodatnu prepreku koju neke sajtove žele da izbegnu.

Tu prepreku možemo smanjiti tako da postoji samo jedan tok prijave za ceo vaš sajt.

### Kako da ga dobijem?
Svi tipovi naloga trenutno imaju pristup SSO-u. Međutim, maksimalan broj SSO korisnika varira u zavisnosti od vašeg paketa. Kao i kod drugih funkcionalnosti, Pro planovi i viši pružaju direktnu razvojnu podršku.

Uporedimo opcije, a zatim uđimo u detalje svake od njih.

### Migracija korisnika i komentara

Prilikom migracije sa platforme koja koristi SSO, poput Disqus-a, već ćete imati korisnike i njihove komentare.

Komentari se uvoze kao deo vaše migracije, bilo putem API-ja, našeg Import UI-ja, ili korisničke podrške. Import UI je preferiran ako podržava platformu sa koje
se migrirate, jer uključuje rukovanje greškama, ekstrakciju i otpremanje avatara i medija, kao i sistem za praćenje batch poslova.

Sam korisnici se dodaju automatski prilikom prvog pregledanja niti komentara. Alternativno, mogu se unapred dodati putem API-ja, ali taj posao nema
mnogo prednosti.

Ako se komentari uvezu, a SSO korisnici nisu ručno dodati putem API-ja, komentari će automatski biti migrirani na nalog korisnika prilikom
njegovog prvog kreiranja kada pregleda bilo koju nit komentara. Tada će moći da upravljaju, uređuju i brišu komentare koje su prvobitno napisali.

Automatska migracija se vrši putem email-a ili korisničkog imena. Neke platforme ne pružaju email adrese pri eksportu, poput Disqus-a, pa u tom slučaju koristimo korisničko ime.
- Sve dok prosledite odgovarajuće korisničko ime, i email u SSO payload-u, dodaćemo email pojedinačnim objektima komentara kako bi notifikacije i pominjanja radili.

Ako želite da uvezete komentare i korisnike odjednom, sarađujte sa podrškom da se komentari migriraju na odgovarajuće naloge korisnika nakon što se korisnici uvezu
putem API-ja.

Dakle, da rezimiramo najlakši put za migraciju:

1. Uvezite komentare.
   1. Avatari i ostali mediji se migriraju automatski ako koristite Import UI u `Manage Data -> Imports`.
2. Podesite Secure ili Simple SSO.
3. Dozvolite da migracija obavi automatski po korisniku kada se prvi put prijavi.
   1. Ovo obično dodaje manje od sekunde vremenu učitavanja stranice ako korisnik ima manje od 50k komentara.

### WordPress korisnici
Ako koristite naš <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress plugin</a>, ne morate pisati kod! Jednostavno idite na Admin stranicu plugina, kliknite SSO Settings, a zatim Enable.

Ovo će vas odvesti na čarobnjak sa jednim klikom dugmetom koji će kreirati vaš API ključ, poslati ga na vašu WordPress instalaciju i uključiti SSO. Ovo smo konsolidovali u jedan klik dugmeta za vas.

Imajte na umu da ako instalirate plugin prvi put, moraćete da završite proces podešavanja pre nego što vidite admin stranicu sa dugmetom SSO Settings.

#### WordPress SSO - Moderatori

Imajte na umu da trenutno, kako bi se "Moderator" bedž prikazao pored vaših moderatora kada komentiraju putem FastComments WordPress plugina,
oni takođe moraju biti dodati kao Moderator u FastComments kontrolnoj tabli, i imati verifikovan email.

### Prilagođene integracije

Za prilagođene integracije, postoje dve opcije.

### Opcija jedan - Secure SSO

Sa Secure SSO-om, FastComments zna da korisnik koji komentariše, glasa i čita komentare je stvaran korisnik na vašem sajtu.

Sve dok kreirate validan payload, korisnik će uvek imati besprekorno iskustvo komentiranja.

Sa Secure SSO-om, SSO payload se kreira **server-side** koristeći HMAC autentikaciju, a zatim se prosleđuje vidžetu na **klijentu**.

Sa Secure SSO-om, nalog korisnika je **potpuno odvojen** od ostatka FastComments korisničke baze. To znači da ako imamo dva partnera,
Kompanija A i Kompanija B, svaka može imati SSO korisnika sa korisničkim imenom "Bob".

#### Zahtevi
- Osnovno znanje o backend razvoju.
- Osnovno znanje o radu sa tajnim API ključevima.
- Osnovno znanje o API razvoju ili server-side renderovanju.

#### Prednosti
- Bezbedno.
- Besprekorno iskustvo komentiranja.

#### Nedostaci
- Zahteva backend razvoj.

#### Ažuriranje podataka korisnika

Sa Secure SSO-om, svaki put kada prosledite sso user payload, ažuriraćemo njihovog korisnika sa najnovijim informacijama. Na primer, ako
korisnik ima korisničko ime `X`, a vi prosledite `Y` u SSO payload-u, njihovo korisničko ime će postati `Y`.

Ako želite da uklonite vrednosti ovim pristupom, postavite ih na `null` (ne `undefined`).

#### Secure SSO API

Takođe pružamo API za interakciju sa SSO korisnicima. Pogledajte [dokumentaciju](/guide-api.html#sso-user-structure).

Imajte na umu da prilikom korišćenja Secure SSO-a, korisnici se automatski kreiraju u pozadini pri učitavanju stranice. Ne morate masovno uvoziti svoje korisnike.

### Opcija dva - Simple SSO

Alternativa Secure SSO-u je jednostavno prosleđivanje informacija o korisniku vidžetu za komentiranje.

Prosleđivanje email-a sa Simple SSO-om nije obavezno, međutim bez toga njihovi komentari će se prikazivati kao "Unverified".

<sup>Napomena!</sup> Od početka 2022. korisnička imena sa Simple SSO-om ne moraju biti jedinstvena na celom FastComments.com.

Idealno, Simple SSO bi trebalo birati samo prilikom razvoja na platformi koja ne pruža backend pristup.

#### Zahtevi
- Osnovno znanje o client-side razvoju.
- Potrebno je znati barem email korisnika.

#### Prednosti
- Jednostavno.
- Sva aktivnost se i dalje verifikuje.
- Korisnik nikada ne unosi svoje korisničko ime ili email.

#### Nedostaci
- Manje bezbedno od Secure SSO-a jer client-side payload može biti falsifikovan da postane bilo koji korisnik.

#### Simple SSO API

Korisnici automatski kreirani putem Simple SSO toka čuvaju se kao `SSOUser` objekti. Mogu se pristupiti i upravljati njima putem `SSOUser` API-ja. Pogledajte [dokumentaciju](/guide-api.html#sso-user-structure).