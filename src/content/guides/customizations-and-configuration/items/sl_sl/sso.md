SSO, oziroma single-sign-on, je niz konvencij, ki omogoča vam ali vašim uporabnikom uporabo FastComments brez potrebe po ustvarjanju dodatnega računa.

Če ne dovolite anonimnega komentiranja, je za komentiranje z FastComments potreben račun. Ta postopek prijave naredimo zelo enostaven – uporabnik ob komentiranju pusti samo svoj e-poštni naslov.
Vendar razumemo, da je tudi to lahko dodatna ovira, ki jo nekatere strani želijo odpraviti.

To trenuten trenutek zmanjšamo tako, da za celotno spletno mesto uporabi samo en prijavni tok.

### Kako ga dobim?
Vse vrste računov trenutno dobijo dostop do SSO. Vendar se največje število SSO uporabnikov razlikuje glede na vaš paket. Tako kot pri drugih funkcijah, tudi načrti Pro in višji nudijo neposredno razvojno podporo.

Primerjajmo možnosti in se nato podrobno lotimo vsake izmed njih.

### Migracije uporabnikov in komentarjev

Pri migraciji z platforme, ki podpira SSO, kot je Disqus, boste že imeli uporabnike in njihove komentarje.

Komentarji so uvoženi kot del migracije, bodisi preko API-ja, našega uvoznega vmesnika (Import UI) ali s podporo strankam. Import UI je prednostna izbira, če podpira platformo, iz katere migrirate, saj vključuje obravnavo napak, izvoz in nalaganje avatarjev in medijskih vsebin ter sistem za spremljanje serijskih opravil.

Samo uporabniki se dodajo samodejno ob prvem ogledu nitk komentarjev. Alternativno jih je mogoče predhodno dodati preko API-ja, vendar to delo ne prinaša veliko prednosti.

Če so komentarji uvoženi in SSO uporabniki niso ročno dodani preko API-ja, bodo komentarji samodejno pripisani uporabnikovemu računu ob prvem
ustvarjanju tega računa, ko si ogleda katerokoli nit komentarjev. Nato bodo lahko upravljali, urejali in brisali komentarje, ki so jih prvotno napisali.

Samodejna migracija poteka preko e-pošte ali uporabniškega imena. Nekatere platforme ob izvozu ne zagotavljajo e-poštnih naslovov, kot je Disqus, zato v tem primeru uporabimo uporabniško ime.
- Dokler posredujete ujemajoče se uporabniško ime in e-poštni naslov v SSO obremenitvi, bomo e-poštni naslov dodali posameznim objektom komentarjev, tako da bodo obvestila in omembe delovale.

Če želite uvoziti komentarje in uporabnike hkrati, sodelujte s podporo, da prebijejo komentarje na ustrezne račune uporabnikov potem, ko so uporabniki uvoženi
preko API-ja.

Torej, da povzemo najlažjo pot za migracijo:

1. Uvozite komentarje.
   1. Avatarji in druga medijska vsebina se samodejno uvozijo, če uporabljate Import UI v `Manage Data -> Imports`.
2. Nastavite Secure ali Simple SSO.
3. Naj se migracija izvede za vsakega uporabnika samodejno, ko se prvič prijavi.
   1. To običajno doda manj kot sekundo k času nalaganja strani, če ima uporabnik manj kot 50k komentarjev.

### Uporabniki WordPressa
Če uporabljate naš <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress vtičnik</a>, potem ni treba pisati nobene kode! Preprosto pojdite na stran skrbnika vtičnika, kliknite SSO Settings in nato Enable.

To vas pripelje do čarovnika z enim klikom, ki bo ustvaril vaš API ključ, ga poslal na vašo WordPress namestitev in vklopil SSO. To smo za vas združili v en sam klik.

Upoštevajte, da če nameščate vtičnik prvič, boste morali dokončati postopek namestitve, preden boste videli stran skrbnika z gumbom SSO Settings.

#### WordPress SSO - Moderatorji

Upoštevajte, da se zaenkrat, da bi se znašla značka "Moderator" ob komentarjih vaših moderatorjev, ko komentirajo z FastComments WordPress vtičnikom,
morajo biti ti tudi dodani kot Moderator v nadzorni plošči FastComments in imeti preverjen e-poštni naslov.

### Prilagojene integracije

Za prilagojene integracije sta na voljo dve možnosti.

### Možnost ena - Secure SSO

S Secure SSO FastComments ve, da je uporabnik, ki komentira, glasuje in bere komentarje, resničen uporabnik na vašem spletnem mestu.

Dokler ustvarite veljaven payload, bo uporabnik vedno imel nemoteno izkušnjo komentiranja.

Pri Secure SSO se SSO payload ustvari na **strežniku** z uporabo HMAC avtentikacije in se nato posreduje v pripomoček na **odjemalcu**.

Pri Secure SSO je uporabnikov račun **popolnoma ločen** od preostale baze uporabnikov FastComments. To pomeni, da če imamo dva partnerja
Firma A in Firma B, lahko imata oba SSO uporabnika z uporabniškim imenom "Bob".

#### Zahteve
- Nekaj osnovnega znanja o strežniškem razvoju.
- Nekaj osnovnega znanja o ravnanju s skrivnimi API ključi.
- Nekaj osnovnega znanja o razvoju API-jev ali strežniškem upodabljanju.

#### Prednosti
- Varen.
- Nemotena izkušnja komentiranja.

#### Slabosti
- Zahteva razvoj na strežniku.

#### Posodabljanje podatkov uporabnika

Pri Secure SSO bomo ob vsakem posredovanju sso uporabniškega payloada posodobili njihov uporabniški račun z najnovejšimi informacijami. Na primer, če
ima uporabnik uporabniško ime `X`, in v SSO payloadu posredujete `Y`, bo njihovo uporabniško ime postalo `Y`.

Če s tem pristopom želite odstraniti vrednosti, jih nastavite na `null` (ne `undefined`).

#### Secure SSO API

Nudimo tudi API za interakcijo s SSO uporabniki. Oglejte si [the docs](/guide-api.html#sso-user-structure).

Upoštevajte, da se pri uporabi Secure SSO uporabniki ob nalaganju strani samodejno ustvarijo v ozadju. Ni vam treba množično uvažati uporabnikov.

### Možnost dve - Simple SSO

Alternativa Secure SSO je, da preprosto posredujete uporabniške informacije v pripomoček za komentiranje.

Posredovanje e-poštnega naslova pri Simple SSO ni obvezno, vendar bodo brez njega njihovi komentarji prikazani kot "Unverified".

<sup>Opomba!</sup> Od začetka leta 2022 uporabniška imena pri Simple SSO ne potrebujejo biti edinstvena po celotnem FastComments.com.

Idealno naj bi bil Simple SSO izbran le pri razvoju na platformi, ki ne omogoča dostopa do strežniškega dela.

#### Zahteve
- Nekaj osnovnega znanja o razvoju na strani odjemalca.
- Poznavanje vsaj e-poštnega naslova uporabnika.

#### Prednosti
- Preprosto.
- Vsa aktivnost se kljub temu preveri.
- Uporabnik nikoli ne vnese svojega uporabniškega imena ali e-poštnega naslova.

#### Slabosti
- Manj varno kot Secure SSO, saj bi lahko bilo podatkovno breme na strani odjemalca zlonamerno konstruirano tako, da postanete kateri koli uporabnik.

#### Simple SSO API

Uporabniki, samodejno ustvarjeni preko Simple SSO toka, so shranjeni kot `SSOUser` objekti. Dostopate in jih upravljate lahko preko `SSOUser` API-ja. Oglejte si [the docs](/guide-api.html#sso-user-structure).