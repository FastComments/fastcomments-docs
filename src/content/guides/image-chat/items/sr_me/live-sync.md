### Ažuriranja u realnom vremenu

Image Chat koristi WebSocket konekcije za sinhronizaciju svih razgovora u realnom vremenu između svih povezanih korisnika. Kada neko kreira novi označivač, doda komentar ili obriše diskusiju, svi ostali korisnici koji gledaju istu sliku vide ažuriranje odmah bez osvježavanja.

### Kako funkcioniše WebSocket sinhronizacija

Kada inicijalizujete Image Chat, vidžet uspostavlja WebSocket konekciju sa FastComments serverima. Ta konekcija ostaje otvorena tokom trajanja korisničke sesije i osluškuje ažuriranja vezana za trenutnu sliku.

WebSocket sistem koristi tri tipa broadcast poruka za Image Chat. Događaj `new-image-chat` se aktivira kada neko kreira novi marker na slici. Događaj `image-chat-updated` se aktivira kada neko ažurira postojeću konverzaciju. Događaj `deleted-image-chat` se aktivira kada neko obriše marker.

### Sistem Broadcast ID-a

Da bi se spriječili efekti odjeka u kojima korisnici vide svoje sopstvene akcije vraćene nazad, svako ažuriranje uključuje jedinstveni `broadcastId`. Kada korisnik kreira ili ažurira marker, njihov klijent generiše UUID za tu operaciju. Kada WebSocket emitira ažuriranje nazad svim klijentima, klijent koji je inicirao operaciju ignoriše to ažuriranje jer se podudara sa njegovim sopstvenim `broadcastId`.

Ovo osigurava glatku interakciju u kojoj korisnici odmah vide svoje promjene u korisničkom interfejsu bez čekanja za putovanje do servera i nazad, dok istovremeno svi ostali korisnici dobijaju ažuriranje.

### Otpornost konekcije

Ako WebSocket konekcija padne zbog mrežnih problema ili održavanja servera, vidžet automatski pokušava da se ponovo poveže. Tokom perioda ponovnog povezivanja, korisnici i dalje mogu da interaguju sa postojećim markerima, ali neće vidjeti ažuriranja od drugih korisnika dok se konekcija ne uspostavi ponovo.

Kada se konekcija ponovo uspostavi, vidžet se ponovo sinhronizuje kako bi osigurao da nijedno ažuriranje nije propušteno. Ovo se dešava transparentno bez potrebe za intervencijom korisnika.

### Razmatranja propusnosti

WebSocket poruke su lagane i sadrže samo osnovne informacije potrebne za sinhronizaciju stanja. Kreiranje novog označivača obično koristi manje od 1KB propusnosti. Sistem takođe uključuje inteligentno grupisanje kako bi smanjio frekvenciju poruka tokom perioda velike aktivnosti.

Vaši metrički podaci u FastComments kontrolnoj tabli prate `pubSubMessageCount` i `pubSubBandwidth` kako biste mogli pratiti aktivnost real-time sinhronizacije na vašim sajtovima.

### Sinhronizacija među karticama

Ako korisnik ima istu stranicu otvorenu u više browser kartica, ažuriranja u jednoj kartici pojavljuju se odmah u ostalim karticama. Ovo radi kroz isti WebSocket mehanizam i ne zahtijeva dodatnu konfiguraciju.

Korisnici mogu imati vaš sajt otvoren na više uređaja istovremeno, i svi će ostati sinhronizovani. Marker kreiran na desktop računaru se pojavljuje instantno na korisnikovom tabletu ako oba uređaja gledaju istu sliku.

### Sigurnost

WebSocket poruke se prenose preko sigurnih konekcija (WSS) i uključuju validaciju tenant-a kako bi se osiguralo da korisnici dobijaju samo ažuriranja za konverzacije koje su ovlašćeni da vide. Server validira sve operacije prije nego što ih emituje kako bi spriječio neovlašćeni pristup ili manipulaciju.

### Ponašanje u offline režimu

Kada su korisnici potpuno offline, mogu i dalje pregledati postojeće markere ali ne mogu kreirati nove niti vidjeti ažuriranja od drugih. Vidžet detektuje offline stanje i prikazuje odgovarajuću poruku.

Ako korisnik pokuša da kreira marker dok je offline, a zatim se ponovo poveže, operacija će biti neuspešna umjesto da se stavi u red, čime se osigurava konzistentnost podataka. Korisnici bi trebali ponoviti operaciju kada im se veza obnovi.

### Uticaj na performanse

WebSocket konekcija ima minimalan uticaj na performanse. Konekcija ostaje u mirovanju kada nema ažuriranja i procesira poruke samo kada se dogode aktivnosti. Na tipičnoj slici sa umjerenom aktivnošću markera, WebSocket koristi manje CPU resursa nego sam rendering slike.

Za stranice sa stotinama istovremenih korisnika i velikom aktivnosti kreiranja markera, sistem se horizontalno skalira kako bi održao performanse bez uticaja na pojedinačne klijentske konekcije.

### Kolaborativni slučajevi upotrebe

Sinhronizacija u realnom vremenu čini Image Chat naročito moćnim za kolaborativne tokove rada. Dizajn timovi mogu zajedno pregledati makete uz to da svi vide postavke markera u realnom vremenu. Timovi za korisničku podršku mogu zajednički anotirati snimke ekrana kako bi identifikovali probleme. Edukativne grupe mogu raspravljati o dijagramima pri čemu svi učesnici vide markere drugih kako se kreiraju.

Neposredna povratna informacija stvara angažovanije i produktivnije kolaborativno iskustvo u poređenju sa tradicionalnim sistemima komentara gdje korisnici moraju osvježiti stranicu da bi vidjeli ažuriranja.