SSO, odnosno single-sign-on, je skup konvencija koji omogućava vama ili vašim korisnicima da koriste FastComments bez potrebe za kreiranjem još jednog naloga.

Pod pretpostavkom da ne dozvoljavate anonimno komentarisanje, nalog je potreban da bi se komentarisalo sa FastComments. Mi ovaj proces prijave činimo vrlo jednostavnim — korisnik samo ostavi svoj email kada komentariše.
Međutim, razumemo da je čak i to dodatna prepreka koju neke stranice žele da izbegnu.

Možemo smanjiti tu prepreku tako što ćemo imati samo jedan tok prijave za ceo vaš sajt.

### How do I get it?
Svi tipovi naloga trenutno dobijaju pristup SSO. Međutim, maksimalan broj SSO korisnika varira u zavisnosti od vašeg paketa. Kao i kod drugih funkcija, Pro planovi i viši obezbeđuju direktnu razvojnu podršku.

Uporedićemo opcije, a zatim ćemo ući u detalje svake.

### User and Comment Migrations

Kada migrirate sa platforme koja ima SSO, poput Disqus, već ćete imati korisnike i njihove komentare.

Komentari se uvoze kao deo vaše migracije, bilo preko API-ja, našeg Import UI ili korisničke podrške. Import UI je poželjan ako podržava platformu sa koje migrirate, jer uključuje rukovanje greškama, ekstrakciju i upload avatara i medija, kao i sistem praćenja batch poslova.

Sami korisnici se dodaju automatski prilikom prvog pregleda niti komentara. Alternativno, mogu biti unapred dodati preko API-ja, ali taj rad nema mnogo prednosti.

Ako su komentari importovani, a SSO korisnici nisu ručno dodatii preko API-ja, komentari će biti automatski dodeljeni korisničkom nalogu prvi put kada se nalog kreira kada korisnik pogleda bilo koju nit komentara. Oni će tada moći da upravljaju, uređuju i brišu komentare koje su prvobitno napisali.

Automatska migracija se radi putem email-a ili korisničkog imena. Neke platforme ne obezbeđuju emailove pri exportu, kao Disqus, pa se u tom slučaju oslanjamo na korisničko ime.
- Sve dok prosledite odgovarajuće korisničko ime i email u SSO payload-u, dodaćemo email u pojedinačne objekte komentara tako da obaveštenja i pominjanja funkcionišu.

Ako želite da importujete komentare i korisnike odjednom, radite sa podrškom kako biste preneli komentare na odgovarajuće korisničke naloge nakon što su korisnici importovani
preko API-ja.

Dakle, da sumiramo najlakši put za migraciju:

1. Importujte komentare.
   1. Avatari i drugi mediji se automatski migriraju ako koristite Import UI u `Manage Data -> Imports`.
2. Podesite Secure ili Simple SSO.
3. Dozvolite da se migracija desi po korisniku automatski kada se prvi put prijave.
   1. Ovo obično dodaje manje od sekunde na vreme učitavanja stranice ako korisnik ima manje od 50k komentara.

### WordPress Users
Ako koristite naš <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress dodatak</a>, nema potrebe za pisanjem koda! Jednostavno idite na administratorsku stranicu dodatka, kliknite SSO Settings, i zatim Enable.

Ovo će vas odvesti do čarobnjaka sa jednim klikom koji će kreirati vaš API ključ, poslati ga na vašu WordPress instalaciju i uključiti SSO. Konsolidovali smo ovo u jedan klik za vas.

Obratite pažnju da ako instalirate dodatak po prvi put moraćete da završite proces podešavanja pre nego što vidite administratorsku stranicu sa dugmetom SSO Settings.

#### WordPress SSO - Moderators

Obratite pažnju da trenutno, da bi se bedž "Moderator" prikazao pored vaših moderatora kada komentarišu koristeći FastComments WordPress dodatak,
oni takođe moraju biti dodati kao Moderator u FastComments kontrolnoj tabli, i imati svoj email verifikovan.

### Custom Integrations

Za prilagođene integracije postoje dve opcije.

### Option One - Secure SSO

Sa Secure SSO, FastComments zna da je korisnik koji komentariše, glasa i čita komentare stvarni korisnik na vašem sajtu.

Sve dok kreirate validan payload, korisnik će uvek imati besprekorno iskustvo komentarisanja.

Sa Secure SSO, SSO payload se kreira server-side koristeći HMAC autentifikaciju i zatim se prosleđuje widgetu na client-u.

Sa Secure SSO, korisnički nalog je potpuno odvojen od ostatka FastComments baze korisnika. To znači da ako imamo dva partnera
Company A i Company B, svaki može imati SSO korisnika sa korisničkim imenom "Bob".

#### Requirements
- Osnovno znanje o backend razvoju.
- Osnovno znanje o rukovanju tajnim API ključevima.
- Osnovno znanje o razvoju API-ja ili server-side renderovanju.

#### Pros
- Sigurno.
- Besprekorno iskustvo komentarisanja.

#### Cons
- Zahteva razvoj na backend-u.

#### Updating User Data

Sa Secure SSO, svaki put kada prosledite sso user payload, mi ćemo ažurirati njihov nalog najnovijim informacijama. Na primer, ako
korisnik ima korisničko ime `X`, i vi prosledite `Y` u SSO payload-u, njihovo korisničko ime će postati `Y`.

Ako želite da uklonite vrednosti koristeći ovaj pristup, postavite ih na `null` (ne `undefined`).

#### Secure SSO API

Takođe obezbeđujemo API za interakciju sa SSO korisnicima. Pogledajte [the docs](/guide-api.html#sso-user-structure).

Napomena da kada koristite Secure SSO, korisnici se automatski kreiraju u pozadini prilikom učitavanja stranice. Ne morate masovno importovati svoje korisnike.

### Option Two - Simple SSO

Alternativa Secure SSO je jednostavno prosleđivanje informacija o korisniku widgetu za komentarisanje.

Nije obavezno proslediti email sa Simple SSO, međutim bez njega njihovi komentari će se prikazivati kao "Unverified".

<sup>Napomena!</sup> Od početka 2022. korisnička imena sa Simple SSO ne moraju biti jedinstvena preko celog FastComments.com.

Idealno, Simple SSO bi trebalo da se bira samo kada razvijate na platformi koja ne obezbeđuje pristup backend-u.

#### Requirements
- Osnovno znanje o client-side razvoju.
- Morate bar poznavati email korisnika.

#### Pros
- Jednostavno.
- Sva aktivnost je i dalje verifikovana.
- Korisnik nikada ne unosi svoje korisničko ime ili email.

#### Cons
- Manje sigurno od Secure SSO jer se client-side payload može falsifikovati da postane bilo koji korisnik.

#### Simple SSO API

Korisnici koji su automatski kreirani putem Simple SSO toka sačuvani su kao `SSOUser` objekti. Može im se pristupiti i upravljati preko `SSOUser` API-ja. Pogledajte [the docs](/guide-api.html#sso-user-structure).