### Real-Time Updates

Collab Chat koristi WebSocket veze za sinhronizaciju svih razgovora u realnom vremenu između svih povezanih korisnika. Kada neko kreira novu anotaciju, doda komentar ili obriše diskusiju, svi ostali korisnici koji gledaju istu stranicu vide ažuriranje odmah bez osvežavanja.

### How WebSocket Sync Works

Kada inicijalizujete Collab Chat, vidžet uspostavlja WebSocket vezu ka FastComments serverima. Ova veza ostaje otvorena tokom trajanja sesije korisnika i sluša ažuriranja vezana za trenutnu stranicu.

WebSocket sistem koristi tri tipa broadcast poruka za Collab Chat. Događaj `new-text-chat` se aktivira kada neko kreira novu anotaciju na stranici. Događaj `updated-text-chat` se aktivira kada neko ažurira postojeći razgovor. Događaj `deleted-text-chat` se aktivira kada neko obriše anotaciju.

### Broadcast ID System

Da bi se sprečili efekti odjeka gde korisnici vide sopstvene akcije vraćene nazad njima, svako ažuriranje uključuje jedinstveni `broadcastId`. Kada korisnik kreira ili ažurira anotaciju, njihov klijent generiše UUID za tu operaciju. Kada WebSocket emituje ažuriranje nazad svim klijentima, originalni klijent ignoriše ažuriranje jer se poklapa sa njegovim `broadcastId`.

Ovo obezbeđuje glatku interakciju gde korisnici odmah vide svoje promene u UI bez čekanja na povratno slanje kroz server, a istovremeno svi ostali korisnici dobijaju ažuriranje.

### Live User Count

Gornja traka prikazuje broj korisnika koji trenutno gledaju stranicu. Ovaj broj se ažurira u realnom vremenu kako se korisnici pridružuju i napuštaju. Broj korisnika se obezbeđuje kroz istu WebSocket vezu i automatski se povećava/smanjuje na osnovu događaja povezivanja i prekida veze.

### Connection Resilience

Ako WebSocket veza padne zbog problema u mreži ili održavanja servera, vidžet automatski pokušava da se ponovo poveže. Tokom perioda ponovnog povezivanja, korisnici i dalje mogu da interaguju sa postojećim anotacijama, ali neće videti ažuriranja od drugih korisnika dok veza ne bude ponovo uspostavljena.

Kada se veza ponovo uspostavi, vidžet se resinhronizuje kako bi se osiguralo da nijedno ažuriranje nije propušteno. Ovo se dešava transparentno bez potrebe za intervencijom korisnika.

### Bandwidth Considerations

WebSocket poruke su lagane i sadrže samo osnovne informacije potrebne za sinhronizaciju stanja. Kreiranje nove anotacije obično koristi manje od 1KB propusnog opsega. Sistem takođe uključuje inteligentno grupisanje kako bi smanjio učestalost poruka tokom perioda visokog intenziteta aktivnosti.

Vaši metrički podaci korišćenja u FastComments dashboard prate `pubSubMessageCount` i `pubSubBandwidth` kako biste mogli da nadgledate aktivnost real-time sinhronizacije na vašim sajtovima.

### Cross-Tab Synchronization

Ako korisnik ima istu stranicu otvorenu u više kartica pregledača, ažuriranja u jednoj kartici se pojavljuju odmah u ostalim karticama. Ovo funkcioniše kroz isti WebSocket mehanizam sinhronizacije i ne zahteva dodatnu konfiguraciju.

### Security

WebSocket poruke se prenose preko sigurnih veza (WSS) i uključuju validaciju tenant-a kako bi se osiguralo da korisnici primaju samo ažuriranja za razgovore za koje su ovlašćeni. Server validira sve operacije pre nego što ih emituje kako bi sprečio neovlašćen pristup ili manipulaciju.