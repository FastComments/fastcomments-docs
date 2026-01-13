### Real-Time Updates

Image Chat koristi WebSocket veze za sinhronizaciju svih razgovora u stvarnom vremenu između svih povezanih korisnika. Kada neko kreira novi označivač, doda komentar ili obriše diskusiju, svi ostali korisnici koji gledaju istu sliku vide ažuriranje odmah bez osvježavanja.

### How WebSocket Sync Works

Kada inicijalizujete Image Chat, widget uspostavlja WebSocket vezu sa FastComments serverima. Ova veza ostaje otvorena tokom trajanja korisničke sesije i sluša ažuriranja vezana za trenutnu sliku.

WebSocket sistem koristi tri tipa broadcast poruka za Image Chat. Događaj `new-image-chat` se aktivira kada neko kreira novi označivač na slici. Događaj `image-chat-updated` se aktivira kada neko ažurira postojeću konverzaciju. Događaj `deleted-image-chat` se aktivira kada neko obriše označivač.

### Broadcast ID System

Da bi se spriječili echo efekti gdje korisnici vide svoje vlastite akcije kako im se ponovo emituju, svako ažuriranje uključuje jedinstveni `broadcastId`. Kada korisnik kreira ili ažurira označivač, njihov klijent generiše UUID za tu operaciju. Kada WebSocket emituje ažuriranje nazad svim klijentima, izvornući klijent ignoriše ažuriranje jer se podudara sa njegovim `broadcastId`.

Ovo osigurava glatku interakciju gdje korisnici vide svoje promjene odmah u UI bez čekanja na povratni put preko servera, dok istovremeno svi ostali korisnici dobijaju ažuriranje.

### Connection Resilience

Ako WebSocket veza padne zbog problema sa mrežom ili održavanja servera, widget automatski pokušava ponovo da se poveže. Tokom perioda ponovnog povezivanja, korisnici i dalje mogu da rade sa postojećim označivačima, ali neće vidjeti ažuriranja od drugih korisnika dok se veza ne uspostavi ponovo.

Kada se veza ponovo uspostavi, widget ponovo sinhronizuje kako bi osigurao da nijedno ažuriranje nije propušteno. Ovo se dešava transparentno bez potrebe za intervencijom korisnika.

### Bandwidth Considerations

WebSocket poruke su lagane i sadrže samo osnovne informacije potrebne za sinhronizaciju stanja. Kreiranje novog označivača obično koristi manje od 1KB propusnog opsega. Sistem takođe uključuje inteligentno grupisanje poruka kako bi smanjio frekvenciju poruka tokom perioda visoke aktivnosti.

Vaše metrike upotrebe u FastComments nadzornoj ploči prate `pubSubMessageCount` i `pubSubBandwidth` tako da možete pratiti aktivnost real-time sinhronizacije na vašim sajtovima.

### Cross-Tab Synchronization

Ako korisnik ima istu stranicu otvorenu u više tabova preglednika, ažuriranja u jednom tabu se pojavljuju odmah u ostalim tabovima. Ovo funkcioniše kroz isti WebSocket mehanizam sinhronizacije i ne zahtijeva dodatnu konfiguraciju.

Korisnici mogu imati vaš sajt otvoren na više uređaja istovremeno i svi će ostati sinhronizovani. Oznaka kreirana na desktop računaru pojavljuje se instantno na korisnikovom tabletu ako oba uređaja gledaju istu sliku.

### Security

WebSocket poruke se prenose preko sigurnih veza (WSS) i uključuju validaciju zakupca kako bi se osiguralo da korisnici dobijaju samo ažuriranja za konverzacije koje su ovlašteni vidjeti. Server validira sve operacije prije nego što ih emituje kako bi spriječio neovlašten pristup ili manipulaciju.

### Offline Behavior

Kada su korisnici potpuno van mreže, i dalje mogu pregledavati postojeće označivače, ali ne mogu kreirati nove niti vidjeti ažuriranja od drugih. Widget detektuje offline stanje i prikazuje odgovarajuću poruku.

Ako korisnik pokuša kreirati označivač dok je offline, a zatim se vrati online, operacija će biti neuspješna umjesto da se stavlja u red, čime se osigurava konzistentnost podataka. Korisnici bi trebali ponoviti operaciju kada se veza obnovi.

### Performance Impact

WebSocket veza ima minimalan uticaj na performanse. Veza ostaje u stanju mirovanja kada se ne dešavaju ažuriranja i obrađuje poruke samo kada ima aktivnosti. Na tipičnoj slici sa umjerenom aktivnošću označivača, WebSocket koristi manje CPU-a nego prikaz same slike.

Za stranice sa stotinama simultanih korisnika i velikom aktivnošću kreiranja označivača, sistem se horizontalno skaluje kako bi održao performanse bez uticaja na pojedinačne klijentske veze.

### Collaborative Use Cases

Real-time sinhronizacija čini Image Chat posebno moćnim za kolaborativne tokove rada. Dizajnerski timovi mogu zajedno pregledavati mockup-e uz to da svi vide postavke označivača u stvarnom vremenu. Timovi korisničke podrške mogu zajednički anotirati screenshot-ove kako bi identificirali probleme. Edukativne grupe mogu diskutovati dijagrame uz to da svi učesnici vide označivače drugih kako se kreiraju.

Neposredna povratna informacija stvara angažovanije i produktivnije kolaborativno iskustvo u poređenju sa tradicionalnim sistemima komentara gdje korisnici moraju osvježiti stranicu da bi vidjeli ažuriranja.