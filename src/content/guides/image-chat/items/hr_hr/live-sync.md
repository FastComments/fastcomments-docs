### Real-Time Updates

Image Chat koristi WebSocket veze za sinkronizaciju svih razgovora u stvarnom vremenu između svih povezanih korisnika. Kada netko stvori novu oznaku, doda komentar ili obriše raspravu, svi ostali korisnici koji gledaju istu sliku vide ažuriranje odmah, bez osvježavanja.

### How WebSocket Sync Works

Kada inicijalizirate Image Chat, widget uspostavlja WebSocket vezu prema FastComments serverima. Ta veza ostaje otvorena tijekom korisničke sesije i sluša ažuriranja vezana uz trenutnu sliku.

WebSocket sustav koristi tri vrste broadcast poruka za Image Chat. Događaj `new-image-chat` se pokreće kada netko stvori novu oznaku na slici. Događaj `image-chat-updated` se pokreće kada netko ažurira postojeći razgovor. Događaj `deleted-image-chat` se pokreće kada netko obriše oznaku.

### Broadcast ID System

Kako bi se spriječili echo efekti gdje korisnici vide vlastite radnje koje im se ponovno emitiraju, svako ažuriranje uključuje jedinstveni `broadcastId`. Kada korisnik stvori ili ažurira oznaku, njihov klijent generira UUID za tu operaciju. Kad WebSocket emitira ažuriranje natrag svim klijentima, izvorni klijent zanemaruje ažuriranje jer odgovara njegovom vlastitom `broadcastId`.

To osigurava glatku interakciju u kojoj korisnici vide svoje promjene odmah u UI bez čekanja za putovanje do servera i natrag, a istovremeno svi drugi korisnici dobivaju ažuriranje.

### Connection Resilience

Ako WebSocket veza padne zbog mrežnih problema ili održavanja servera, widget automatski pokušava ponovno uspostaviti vezu. Tijekom razdoblja ponovnog povezivanja, korisnici i dalje mogu manipulirati postojećim oznakama, ali neće vidjeti ažuriranja od drugih korisnika dok veza ne bude ponovno uspostavljena.

Kada se veza ponovno uspostavi, widget se resinkronizira kako bi osigurao da nijedno ažuriranje nije propušteno. To se događa transparentno bez potrebe za intervencijom korisnika.

### Bandwidth Considerations

WebSocket poruke su male i sadrže samo esencijalne informacije potrebne za sinkronizaciju stanja. Kreiranje nove oznake obično koristi manje od 1KB bandwidtha. Sustav također uključuje inteligentno grupiranje poruka kako bi smanjio učestalost poruka tijekom razdoblja velike aktivnosti.

Vaše metrike korištenja na FastComments nadzornoj ploči prate `pubSubMessageCount` i `pubSubBandwidth` kako biste mogli nadzirati aktivnost sinkronizacije u stvarnom vremenu preko vaših stranica.

### Cross-Tab Synchronization

Ako korisnik ima istu stranicu otvorenu u više pregledničkih kartica, ažuriranja u jednoj kartici pojavljuju se odmah u drugim karticama. To radi kroz isti WebSocket mehanizam za sinkronizaciju i ne zahtijeva dodatnu konfiguraciju.

Korisnici mogu imati vašu stranicu otvorenu na više uređaja istovremeno, i svi će ostati sinkronizirani. Oznaka kreirana na stolnom računalu pojavljuje se instantno na korisnikovom tabletu ako oba uređaja gledaju istu sliku.

### Security

WebSocket poruke se prenose preko sigurnih veza (WSS) i uključuju validaciju najamnika kako bi se osiguralo da korisnici primaju samo ažuriranja za razgovore koje su ovlašteni vidjeti. Server validira sve operacije prije nego što ih emitira kako bi sprječavao neovlašteni pristup ili manipulaciju.

### Offline Behavior

Kada su korisnici potpuno offline, i dalje mogu pregledavati postojeće oznake, ali ne mogu stvarati nove niti vidjeti ažuriranja od drugih. Widget detektira offline stanje i prikazuje odgovarajuću poruku.

Ako korisnik pokuša stvoriti oznaku dok je offline, a zatim se vrati online, operacija će ne uspjeti umjesto da se stavi na čekanje, čime se osigurava konzistentnost podataka. Korisnici bi trebali pokušati ponovo kad im se veza ponovno uspostavi.

### Performance Impact

WebSocket veza ima minimalan utjecaj na performanse. Veza ostaje neaktivna kad se ne događaju ažuriranja i obrađuje poruke samo kada ima aktivnosti. Na tipičnoj slici s umjerenom aktivnošću oznaka, WebSocket koristi manje CPU-a nego samo renderiranje slike.

Za stranice s stotinama istodobnih korisnika i velikom aktivnošću kreiranja oznaka, sustav se horizontalno skalira kako bi održao performanse bez utjecaja na pojedinačne klijentske veze.

### Collaborative Use Cases

Sinkronizacija u stvarnom vremenu čini Image Chat posebno moćnim za kolaborativne tijekove rada. Timovi za dizajn mogu zajednički pregledavati makete pri čemu svi vide pozicioniranja oznaka u stvarnom vremenu. Timovi za korisničku podršku mogu zajednički anotirati snimke zaslona kako bi identificirali probleme. Edukacijske grupe mogu raspravljati o dijagramima pri čemu svi sudionici vide oznake drugih kako se stvaraju.

Trenutna povratna informacija stvara angažiranije i produktivnije kolaborativno iskustvo u usporedbi s tradicionalnim sustavima komentiranja gdje korisnici moraju osvježiti stranicu da bi vidjeli ažuriranja.