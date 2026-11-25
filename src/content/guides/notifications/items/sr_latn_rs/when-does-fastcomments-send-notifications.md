U FastComments znamo da vas već ima dovoljno notifikacija. Zbog toga preduzimamo određene mere da ograničimo notifikacije koje korisnici primaju, a da ih i dalje držimo u vezi sa svojim zajednicama. Takođe želimo da administratore i moderatore držimo u toku i da ih obavestimo kada je potrebna akcija.

#### Slali bismo notifikacije za sledeće događaje za administratore i moderatore:

- Rezime zajednice (Digest Summary) (frekvencija je konfigurabilna).
- Zahtevi za pomoć i podsetnici zajednice.
- Novi komentari.

#### Za komentatore:

- Kada neko odgovori na vaš komentar (putem emaila).
- Kada ste pomenuti (notifikacija u aplikaciji i emailom).
- Kada neko odgovori u istom threadu (notifikacija u aplikaciji i emailom).
- Kada neko odgovori na child komentar u istom threadu (notifikacija u aplikaciji i emailom).
- Kada neko odgovori na stranicu koju ste pretplatili (notifikacija u aplikaciji i emailom, frekvencija konfigurabilna po pretplati: svakog minuta, na sat, ili dnevno).
- Kada vam stranica koju ste pretplatili šalje puno emailova (20 ili više u toku nedelje), pitamo vas da li i dalje želite tu frekvenciju, najviše jednom nedeljno.
  - Ništa se ne menja osim ako vi to ne promenite. Email sadrži link za prelazak na dnevne notifikacije.
- Kada neko odgovori na anketu (poll) koju ste kreirali (notifikacija u aplikaciji, plus dnevni email). Vidite [Poll Notifications](/guide-polls.html#poll-notifications).
- Kada korisnik komentariše prvi put (Ali ne sa SSO).
- Kada korisnik ostavi komentar u sesiji koja nije verifikovana (Ali ne sa SSO).
  - U ovom slučaju ne šaljemo više verifikacionih emailova. Samo prvi, koji će verifikovati svu aktivnost u istoj sesiji.

#### Za sve korisnike:

- Kada se detektuje prijava sa nove IP adrese, šalje se email sigurnosni alarm sa približnom lokacijom i IP adresom. Ovo ne važi za prvu prijavu korisnika.

#### ...i na kraju, samo za administratore:

- Kada su integracije završene.
- Kada su migracije završene.
- Kada se import ili export završi.
- Kada postoje problemi sa naplatom.
- Podsetnici na kraj probnog perioda.

Neke notifikacije se grupišu (batch) kako bi se sprečilo masovno slanje notifikacija korisnicima. Saznajte više o tome u sledećem odeljku `Notification Types`.

Emailovi za odgovore i pominjanja se šalju samo za odobrene komentare. Da vidite da li je email za odgovor ili pominjanje poslat za određeni komentar, ili zašto nije, otvorite [Comment Logs](/guide-moderation.html#comment-logs) tog komentara sa stranice Moderate Comments.