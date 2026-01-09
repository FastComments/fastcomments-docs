### Ažuriranja u realnom vremenu

Image Chat koristi WebSocket konekcije za sinhronizaciju svih razgovora u realnom vremenu između svih povezanih korisnika. Kada neko kreira novi marker, doda komentar ili obriše diskusiju, svi ostali korisnici koji gledaju istu sliku vide ažuriranje odmah bez osvežavanja.

### Kako WebSocket sinhronizacija funkcioniše

Kada inicijalizujete Image Chat, vidžet uspostavlja WebSocket konekciju sa FastComments serverima. Ova konekcija ostaje otvorena tokom trajanja korisničke sesije i osluškuje ažuriranja vezana za trenutnu sliku.

WebSocket sistem koristi tri tipa broadcast poruka za Image Chat. Događaj `new-image-chat` se emituje kada neko kreira novi marker na slici. Događaj `image-chat-updated` se emituje kada neko ažurira postojeću konverzaciju. Događaj `deleted-image-chat` se emituje kada neko obriše marker.

### Sistem Broadcast ID

Kako bi se sprečili efekti odjeka gde korisnici vide svoje sopstvene akcije koje im se ponovo emituju, svako ažuriranje uključuje jedinstveni `broadcastId`. Kada korisnik kreira ili ažurira marker, njegov klijent generiše UUID za tu operaciju. Kada WebSocket emituje ažuriranje nazad svim klijentima, klijent koji je inicirao operaciju ignoriše ažuriranje jer odgovara njegovom `broadcastId`.

Ovo obezbeđuje nesmetanu interakciju gde korisnici vide svoje izmene odmah u UI bez čekanja na povratni krug kroz server, dok istovremeno svi drugi korisnici dobijaju ažuriranje.

### Otpornost konekcije

Ako WebSocket konekcija padne zbog mrežnih problema ili održavanja servera, vidžet automatski pokušava da se ponovo poveže. Tokom perioda ponovnog povezivanja, korisnici i dalje mogu da interaguju sa postojećim markerima, ali neće videti ažuriranja od drugih korisnika dok se konekcija ne uspostavi ponovo.

Kada se ponovo poveže, vidžet se resinhronizuje kako bi se osiguralo da nijedno ažuriranje nije propušteno. Ovo se dešava transparentno bez potrebe za intervencijom korisnika.

### Propusni opseg

WebSocket poruke su lagane i sadrže samo neophodne informacije potrebne za sinhronizaciju stanja. Kreiranje novog markera obično koristi manje od 1KB propusnog opsega. Sistem takođe uključuje inteligentno grupisanje poruka kako bi se smanjila učestalost poruka tokom perioda visoke aktivnosti.

Vaši metrički podaci u FastComments kontrolnoj tabli prate `pubSubMessageCount` i `pubSubBandwidth` tako da možete pratiti aktivnost sinhronizacije u realnom vremenu na vašim sajtovima.

### Sinhronizacija između tabova

Ako korisnik ima istu stranicu otvorenu u više browser tabova, ažuriranja u jednom tabu pojavljuju se odmah i u ostalim tabovima. Ovo funkcioniše kroz isti WebSocket mehanizam sinhronizacije i ne zahteva dodatnu konfiguraciju.

Korisnici mogu imati vaš sajt otvoren na više uređaja istovremeno, i svi će ostati sinhronizovani. Marker kreiran na desktop računaru pojavljuje se instantno na korisnikovom tabletu ako oba uređaja gledaju istu sliku.

### Bezbednost

WebSocket poruke se prenose preko sigurnih konekcija (WSS) i uključuju tenant validaciju da bi se osiguralo da korisnici primaju samo ažuriranja za konverzacije koje su ovlašćeni da vide. Server validira sve operacije pre nego što ih emituje kako bi se sprečio neovlašćeni pristup ili manipulacija.

### Ponašanje u offline režimu

Kada su korisnici potpuno offline, i dalje mogu da vide postojeće markere ali ne mogu da kreiraju nove niti da vide ažuriranja od drugih. Vidžet detektuje offline stanje i prikazuje odgovarajuće poruke.

Ako korisnik pokuša da kreira marker dok je offline, a zatim se ponovo poveže, operacija će neuspeti umesto da se stavi u red, čime se obezbeđuje konzistentnost podataka. Korisnici treba ponovo da pokušaju operaciju kada im se veza obnovi.

### Uticaj na performanse

WebSocket konekcija ima minimalan uticaj na performanse. Konekcija ostaje u stanju mirovanja kada nema ažuriranja i procesuira poruke samo kada se aktivnost dogodi. Na tipičnoj slici sa umerenim brojem markera, WebSocket koristi manje CPU resursa nego prikaz same slike.

Za stranice sa stotinama istovremenih korisnika i visokom aktivnošću kreiranja markera, sistem horizontalno skalira kako bi održao performanse bez uticaja na pojedinačne klijentske veze.

### Kolaborativni slučajevi upotrebe

Sinhronizacija u realnom vremenu čini Image Chat posebno moćnim za kolaborativne tokove rada. Dizajn timovi mogu zajedno pregledati makete uz to da svi vide pozicioniranje markera u realnom vremenu. Timovi za korisničku podršku mogu kolektivno anotirati snimke ekrana kako bi identifikovali probleme. Edukativne grupe mogu diskutovati dijagrame uz to da svi učesnici vide marker-e drugih kako se kreiraju.

Neposredna povratna informacija stvara angažovanije i produktivnije kolaborativno iskustvo u poređenju sa tradicionalnim sistemima komentarisanja gde korisnici moraju da osveže stranicu da bi videli ažuriranja.