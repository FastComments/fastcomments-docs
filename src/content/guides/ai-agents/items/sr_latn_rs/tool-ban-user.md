Alat za zabranu je najkonsekventnija akcija koju agent može pozvati. On zabranjuje korisnika iz vaše zajednice, sa fiksnim trajanjem i nekoliko opcija.

### Šta radi

Agent bira jedno od šest trajanja:

- Jedan sat
- Jedan dan
- Jedna nedelja
- Jedan mesec
- Šest meseci
- Jedna godina

Takođe bira između **vidljive zabrane** (korisnik vidi jasnu poruku o zabrani i može podneti žalbu) i **skrivene zabrane** (korisnik može nastaviti da objavljuje, ali njihov sadržaj je skriven od drugih korisnika). Instrukcije platforme nalažu agentu da daje prednost vidljivim zabranama za prve ili granične slučajeve, a skrivenim zabranama za jasno zlonamerne ponavljače.

### Dve destruktivne pod-opcije

Dve dodatne opcije su **podrazumevano skrivene od agenta**. Da biste omogućili bilo koju, označite odgovarajući checkbox u odeljku **Opcije zabrane** na obrascu za izmenu agenta:

- **Allow deleting all of the user's comments.** Kada je omogućeno, agent može izabrati i da obriše sve komentare koje je zabranjeni korisnik ikada objavio u vašem tenant-u. Koristiti samo za očigledni spam, doxxing ili koordinisano zlostavljanje gde postojeći sadržaj nema vrednost. **Destruktivno i nepovratno.**
- **Allow banning by IP.** Kada je omogućeno, agent može izabrati i da zabrani IP sa kojeg je komentar postavljen. Korisno protiv izbegavanja zabrane pomoću alternativnih naloga. **Izbegavati za deljene IP adrese** (korporativne, školske, mobilni operatori) - nevini korisnici na istoj mreži biće blokirani.

Platforma takođe primenjuje ova ograničenja na serverskoj strani: čak i ako agent postane zlonameran i pokuša da pozove opciju, zahtev će biti odbijen osim ako ste eksplicitno pristali.

### Politika eskalacije

Pre nego što zabrani, platforma instrukcijama nalaže agentu da:

1. Pretraži [memoriju agenta](#agent-memory-system) za prethodna upozorenja ili beleške o korisniku.
2. Daje prednost [upozorenju](#tool-warn-user) korisnika umesto zabrane za prve prekršaje.
3. Preskoči korak upozorenja samo za očigledno teške slučajeve (ilegalni sadržaj, doxxing, koordinisani spam) - i objasni zašto u svom opravdanju.

Ova politika je u instrukcijama agenta, a ne strogo pravilo na serverskoj strani, zbog čega se **snažno preporučuje da zabrane zahtevaju odobrenje**.

### Region EU: potrebno ljudsko odobrenje

U regionu EU, ovaj alat je **zaključan tako da zahteva odobrenje** prema članu 17 Zakona o digitalnim uslugama (Digital Services Act). Svaka zabrana od bilo kog agenta na tenant-u u regionu EU završi u [inboxu za odobrenja](#approval-workflow) za ljudski pregled. Pogledajte [Usklađenost sa članom 17 EU DSA](#eu-dsa-compliance).

### Preporuke

- Zahtevajte odobrenje svuda najmanje tokom prvog meseca.
- Uvek stavite opciju **delete-all-comments** pod odobrenje ako je omogućite - to je nepovratno.
- Razmotrite stavljanje opcije **zabrana po IP adresi** pod odobrenje čak i nakon što agent stekne poverenje - trošak zabrane po IP adresi na deljenoj mreži se ne vidi u istoriji izvršavanja agenta.

### Pogledajte i

- [Zabrana korisnika](/guide-moderation.html#banning-users) i [Zabrana korisnika uz korišćenje wildcard-a](/guide-moderation.html#banning-users-wildcards) u vodiču za moderaciju za to kako zabrane funkcionišu na nivou platforme.
- [Upozori korisnika](#tool-warn-user) - blaži korak eskalacije.