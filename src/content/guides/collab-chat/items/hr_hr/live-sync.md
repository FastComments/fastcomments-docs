### Ažuriranja u stvarnom vremenu

Collab Chat koristi WebSocket veze za sinkronizaciju svih razgovora u stvarnom vremenu među svim povezanim korisnicima. Kada netko stvori novu bilješku, doda komentar ili obriše raspravu, svi ostali korisnici koji gledaju istu stranicu odmah vide ažuriranje bez osvježavanja.

### Kako WebSocket sinkronizacija funkcionira

Kad inicijalizirate Collab Chat, widget uspostavlja WebSocket vezu s FastComments poslužiteljima. Ta veza ostaje otvorena tijekom trajanja korisničke sesije i sluša ažuriranja vezana uz trenutnu stranicu.

Sustav WebSocket koristi tri vrste broadcast poruka za Collab Chat. Događaj `new-text-chat` aktivira se kad netko kreira novu bilješku na stranici. Događaj `updated-text-chat` aktivira se kad netko ažurira postojeći razgovor. Događaj `deleted-text-chat` aktivira se kad netko obriše bilješku.

### Sustav Broadcast ID-a

Kako bi se spriječio efekt odjeka u kojem korisnici vide svoje vlastite akcije kako im se vraćaju, svako ažuriranje uključuje jedinstveni `broadcastId`. Kad korisnik stvori ili ažurira bilješku, njegov klijent generira UUID za tu operaciju. Kada WebSocket emitira ažuriranje natrag svim klijentima, izvorni klijent zanemaruje ažuriranje jer odgovara njegovom vlastitom `broadcastId`.

To osigurava glatku interakciju u kojoj korisnici odmah vide svoje promjene u korisničkom sučelju bez čekanja na putovanje podataka kroz poslužitelj, a istovremeno jamči da svi ostali korisnici dobiju ažuriranje.

### Broj korisnika uživo

Gornja traka prikazuje broj korisnika koji trenutno gledaju stranicu. Taj se broj ažurira u stvarnom vremenu kako se korisnici pridružuju i napuštaju. Broj korisnika se prenosi putem iste WebSocket veze i automatski se povećava/smanjuje na temelju događaja povezivanja i prekida veze.

### Otpornost veze

Ako WebSocket veza padne zbog mrežnih problema ili održavanja poslužitelja, widget automatski pokušava ponovno uspostaviti vezu. Tijekom razdoblja ponovnog povezivanja, korisnici i dalje mogu raditi s postojećim bilješkama, ali neće vidjeti ažuriranja u stvarnom vremenu od drugih korisnika dok veza ne bude ponovno uspostavljena.

Nakon ponovnog povezivanja, widget se ponovo sinkronizira kako bi osigurao da nijedno ažuriranje nije propušteno. To se događa transparentno bez potrebe za intervencijom korisnika.

### Razmatranja propusnosti

WebSocket poruke su male i sadrže samo osnovne informacije potrebne za sinkronizaciju stanja. Kreiranje nove bilješke obično koristi manje od 1KB propusnosti. Sustav također uključuje inteligentno grupiranje poruka kako bi se smanjila učestalost poruka tijekom perioda visoke aktivnosti.

Vaše metrike korištenja u FastComments nadzornoj ploči prate `pubSubMessageCount` i `pubSubBandwidth` kako biste mogli pratiti aktivnost sinkronizacije u stvarnom vremenu na vašim stranicama.

### Sinkronizacija između kartica

Ako korisnik ima istu stranicu otvorenu u više pregledničkih kartica, ažuriranja u jednoj kartici pojavljuju se odmah u ostalim karticama. To radi putem istog WebSocket mehanizma sinkronizacije i ne zahtijeva dodatnu konfiguraciju.

### Sigurnost

WebSocket poruke se prenose preko sigurnih veza (WSS) i uključuju provjeru tenanta kako bi se osiguralo da korisnici primaju samo ažuriranja za razgovore za koje su ovlašteni. Poslužitelj validira sve operacije prije njihovog emitiranja kako bi spriječio neovlašteni pristup ili manipulaciju.