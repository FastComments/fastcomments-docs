Alat za zabranu je najznačajnija akcija koju agent može pozvati. On zabrani korisnika iz vaše zajednice, s fiksnim trajanjem i nekoliko opcija.

### Što radi

Agent bira jedno od šest razdoblja:

- Jedan sat
- Jedan dan
- Jedan tjedan
- Jedan mjesec
- Šest mjeseci
- Jedna godina

Također bira između **vidljive zabrane** (korisnik vidi jasnu poruku o zabrani i može uložiti žalbu) i **shadow ban** (korisnik može nastaviti objavljivati, ali njihov sadržaj je skriven od drugih korisnika). Platforma uputama nalaže agentu da daje prednost vidljivim zabranama za prvostruka ili granične slučajeve, a shadow ban za jasno zlonamjerne ponavljače.

### Dvije destruktivne podopcije

Dodatne su dvije opcije **po zadanim postavkama skrivene od agenta**. Da biste omogućili bilo koju od njih, označite odgovarajući potvrdni okvir u odjeljku **Ban options** na obrascu za uređivanje agenta:

- **Allow deleting all of the user's comments.** Kada je omogućeno, agent može odabrati i izbrisati svaki komentar koji je zabranjeni korisnik ikada objavio u vašem tenantu. Ostavite za jasni spam, doxxing ili koordinirano zlostavljanje gdje postojeći sadržaj nema vrijednost. **Destruktivno i nepovratno.**
- **Allow banning by IP.** Kada je omogućeno, agent može odabrati i zabraniti IP s kojeg je komentar objavljen. Korisno protiv izbjegavanja zabrana putem alternativnih računa. **Izbjegavajte za dijeljene IP-ove** (korporativni, školski, mobilni operatori) - nevini korisnici na istoj mreži bit će blokirani.

Platforma također ograničava ove opcije na strani poslužitelja: čak i ako agent poklekne i pokuša pozvati opciju, zahtjev se odbija osim ako se niste izričito prijavili.

### Politika eskalacije

Prije zabrane, platforma uputama nalaže agentu da:

1. Pretraži [memorija agenta](#agent-memory-system) za prethodna upozorenja ili bilješke o korisniku.
2. Za prvostupanjske prekršaje radije [upozori](#tool-warn-user) korisnika nego ga zabrani.
3. Preskoči korak upozorenja samo za jasno teške slučajeve (ilegalni sadržaj, doxxing, koordinirani spam) - i objasni zašto u svom obrazloženju.

Ova politika je u uputama agenta, a ne strogo pravilo na strani poslužitelja, zbog čega se **snažno preporučuje da zabrane budu uvjetovane odobrenjem**.

### EU regija: potrebno je ljudsko odobrenje

U EU regiji, ovaj je alat **zaključen za odobrenje** prema članku 17. Digital Services Act-a. Svaka zabrana od bilo kojeg agenta na tenant-u u EU regiji završava u [inboxu za odobrenja](#approval-workflow) za ljudsku provjeru. Pogledajte [Usklađenost s EU DSA Člankom 17](#eu-dsa-compliance).

### Preporuke

- Postavite zahtjev za odobrenje svugdje barem tijekom prvog mjeseca.
- Uvijek zahtijevajte odobrenje za opciju **delete-all-comments** ako je omogućite - ona je nepovratna.
- Razmotrite zahtijevanje odobrenja i za opciju **IP ban** čak i nakon što agent stekne povjerenje - trošak IP zabrane na dijeljenoj mreži ne prikazuje se u povijesti rada agenta.

### Vidi također

- [Zabrana korisnika](/guide-moderation.html#banning-users) i [Zabrana korisnika s džokerima](/guide-moderation.html#banning-users-wildcards) u vodiču za moderiranje kako zabrane funkcioniraju na cijeloj platformi.
- [Upozori korisnika](#tool-warn-user) - blaži korak eskalacije.