### Ažuriranja u realnom vremenu

Collab Chat koristi WebSocket veze da sinhronizuje sve razgovore u realnom vremenu među svim povezanim korisnicima. Kada neko napravi novu anotaciju, doda komentar ili obriše diskusiju, svi ostali korisnici koji gledaju istu stranicu odmah vide ažuriranje bez osvežavanja.

### Kako funkcioniše WebSocket sinhronizacija

Kada inicijalizujete Collab Chat, vidžet uspostavlja WebSocket vezu sa FastComments serverima. Ova veza ostaje otvorena tokom trajanja korisničke sesije i sluša ažuriranja vezana za trenutnu stranicu.

WebSocket sistem koristi tri tipa broadcast poruka za Collab Chat. Događaj `new-text-chat` se aktivira kada neko napravi novu anotaciju na stranici. Događaj `updated-text-chat` se aktivira kada neko ažurira postojeći razgovor. Događaj `deleted-text-chat` se aktivira kada neko obriše anotaciju.

### Sistem broadcast ID-jeva

Da bi se sprečili efekti odjeka gde korisnici vide sopstvene akcije koje im se vraćaju, svako ažuriranje sadrži jedinstveni `broadcastId`. Kada korisnik kreira ili ažurira anotaciju, njegov klijent generiše UUID za tu operaciju. Kada WebSocket emituje ažuriranje svim klijentima, klijent koji je pokrenuo operaciju ignoriše ažuriranje jer se poklapa sa sopstvenim `broadcastId`.

Ovo obezbeđuje glatku interakciju gde korisnici odmah vide svoje promene u UI-ju bez čekanja na povratni put kroz server, a istovremeno se obezbeđuje da svi ostali korisnici dobiju ažuriranje.

### Broj korisnika uživo

Gornja traka prikazuje broj korisnika koji trenutno gledaju stranicu. Ovaj broj se ažurira u realnom vremenu kako korisnici dolaze i odlaze. Broj korisnika se dostavlja kroz istu WebSocket vezu i automatski se povećava/smanjuje na osnovu događaja povezivanja i prekida veze.

### Otpornost veze

Ako se WebSocket veza prekine zbog problema sa mrežom ili održavanja servera, vidžet automatski pokušava da se ponovo poveže. Tokom perioda ponovnog povezivanja, korisnici i dalje mogu da komuniciraju sa postojećim anotacijama, ali neće videti ažuriranja u realnom vremenu od drugih korisnika dok se veza ne uspostavi ponovo.

Kada se veza ponovo uspostavi, vidžet se resinhronizuje da bi se obezbedilo da nijedno ažuriranje nije promašeno. Ovo se dešava transparentno, bez potrebe za intervencijom korisnika.

### Razmatranja o propusnom opsegu

WebSocket poruke su lagane i sadrže samo osnovne informacije potrebne za sinhronizaciju stanja. Kreiranje nove anotacije obično koristi manje od 1KB propusnog opsega. Sistem takođe uključuje inteligentno grupisanje kako bi smanjio učestalost poruka tokom perioda visoke aktivnosti.

Vaše metrike korišćenja u FastComments kontrolnoj tabli prate `pubSubMessageCount` i `pubSubBandwidth` tako da možete pratiti aktivnost sinhronizacije u realnom vremenu na svojim sajtovima.

### Sinhronizacija između tabova

Ako korisnik ima istu stranicu otvorenu u više tabova pregledača, ažuriranja u jednom tabu se odmah pojavljuju u ostalim tabovima. Ovo funkcioniše kroz isti mehanizam WebSocket sinhronizacije i ne zahteva nikakvu dodatnu konfiguraciju.

### Bezbednost

WebSocket poruke se prenose preko sigurnih veza (WSS) i uključuju validaciju tenanta kako bi se obezbedilo da korisnici primaju samo ažuriranja za razgovore koje imaju ovlašćenje da vide. Server validira sve operacije pre emitovanja kako bi sprečio neovlašćen pristup ili manipulaciju.