### Ažuriranja u realnom vremenu

Collab Chat koristi WebSocket veze za sinhronizaciju svih razgovora u realnom vremenu među svim povezanim korisnicima. Kada neko kreira novu anotaciju, doda komentar ili obriše diskusiju, svi ostali korisnici koji gledaju istu stranicu vide ažuriranje odmah bez osvježavanja.

### Kako WebSocket sinhronizacija radi

Kada inicijalizujete Collab Chat, widget uspostavlja WebSocket vezu sa FastComments serverima. Ta veza ostaje otvorena tokom trajanja korisničke sesije i sluša ažuriranja vezana za trenutnu stranicu.

WebSocket sistem koristi tri tipa broadcast poruka za Collab Chat. `new-text-chat` događaj se pokreće kada neko kreira novu anotaciju na stranici. `updated-text-chat` događaj se pokreće kada neko ažurira postojeći razgovor. `deleted-text-chat` događaj se pokreće kada neko obriše anotaciju.

### Sistem Broadcast ID-a

Da bi se spriječili efekti odjeka gdje korisnici vide svoje vlastite akcije emitovane nazad sebi, svako ažuriranje sadrži jedinstveni `broadcastId`. Kada korisnik kreira ili ažurira anotaciju, njihov klijent generiše UUID za tu operaciju. Kada WebSocket emituje ažuriranje nazad svim klijentima, klijent koji je inicirao akciju ignoriše ažuriranje jer se poklapa sa njegovim vlastitim `broadcastId`.

Ovo osigurava glatku interakciju gdje korisnici vide svoje promjene odmah u UI bez čekanja na povratni put preko servera, a istovremeno osigurava da svi ostali korisnici dobiju ažuriranje.

### Broj korisnika uživo

Gornja traka prikazuje broj korisnika koji trenutno gledaju stranicu. Ovaj broj se ažurira u realnom vremenu dok se korisnici pridružuju i napuštaju. Broj korisnika se prenosi preko iste WebSocket veze i automatski se povećava/smanjuje na osnovu događaja povezivanja i prekida veze.

### Otpornost veze

Ako WebSocket veza padne zbog mrežnih problema ili održavanja servera, widget automatski pokušava ponovo uspostaviti vezu. Tokom perioda ponovnog povezivanja, korisnici i dalje mogu komunicirati sa postojećim anotacijama, ali neće vidjeti ažuriranja u realnom vremenu od drugih korisnika dok veza ne bude ponovo uspostavljena.

Kada se veza ponovo uspostavi, widget vrši resinhronizaciju kako bi osigurao da nijedno ažuriranje nije propušteno. Ovo se dešava transparentno bez potrebe za intervencijom korisnika.

### Razmatranja propusnosti

WebSocket poruke su lake i sadrže samo osnovne informacije potrebne za sinhronizaciju stanja. Kreiranje nove anotacije obično koristi manje od 1KB propusnosti. Sistem takođe uključuje inteligentno grupisanje poruka kako bi se smanjila frekvencija poruka tokom perioda velike aktivnosti.

Vaši metrički podaci u FastComments kontrolnoj tabli prate `pubSubMessageCount` i `pubSubBandwidth` tako da možete pratiti aktivnost sinhronizacije u realnom vremenu na vašim stranicama.

### Sinhronizacija između tabova

Ako korisnik ima istu stranicu otvorenu u više tabova preglednika, ažuriranja u jednom tabu pojavljuju se odmah u ostalim tabovima. Ovo radi preko istog WebSocket mehanizma sinhronizacije i ne zahtijeva dodatnu konfiguraciju.

### Sigurnost

WebSocket poruke se prenose preko sigurnih veza (WSS) i sadrže validaciju zakupca (tenant) kako bi se osiguralo da korisnici primaju samo ažuriranja za razgovore koje su ovlašteni vidjeti. Server validira sve operacije prije nego što ih emitira kako bi spriječio neovlašteni pristup ili manipulaciju.