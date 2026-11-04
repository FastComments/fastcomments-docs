### Ažuriranja u stvarnom vremenu

Image Chat koristi WebSocket veze za sinkronizaciju svih razgovora u stvarnom vremenu među svim povezanim korisnicima. Kada netko stvori novi marker, doda komentar ili izbriše raspravu, svi ostali korisnici koji gledaju istu sliku odmah vide ažuriranje bez osvježavanja.

### Kako funkcionira WebSocket sinkronizacija

Kada inicijalizirate Image Chat, widget uspostavlja WebSocket vezu s FastComments poslužiteljima. Ova veza ostaje otvorena tijekom trajanja korisničke sesije i sluša ažuriranja povezana s trenutnom slikom.

WebSocket sustav koristi tri vrste broadcast poruka za Image Chat. Događaj `new-image-chat` aktivira se kada netko stvori novi marker na slici. Događaj `image-chat-updated` aktivira se kada netko ažurira postojeći razgovor. Događaj `deleted-image-chat` aktivira se kada netko izbriše marker.

### Sustav broadcast ID-jeva

Kako bi se spriječili efekti odjeka gdje korisnici vide vlastite akcije vraćene natrag njima, svako ažuriranje uključuje jedinstveni `broadcastId`. Kada korisnik stvori ili ažurira marker, njegov klijent generira UUID za tu operaciju. Kada WebSocket emitira ažuriranje natrag svim klijentima, izvorni klijent ignorira ažuriranje jer se podudara s vlastitim `broadcastId`.

To osigurava glatku interakciju gdje korisnici odmah vide svoje promjene u sučelju bez čekanja na povratni put kroz poslužitelj, dok se istovremeno osigurava da svi ostali korisnici dobiju ažuriranje.

### Otpornost veze

Ako se WebSocket veza prekine zbog mrežnih problema ili održavanja poslužitelja, widget automatski pokušava ponovno uspostaviti vezu. Tijekom razdoblja ponovnog povezivanja, korisnici i dalje mogu komunicirati s postojećim markerima, ali neće vidjeti ažuriranja u stvarnom vremenu od drugih korisnika dok se veza ne uspostavi ponovno.

Nakon ponovnog povezivanja, widget se resinkronizira kako bi osigurao da nije propušteno nijedno ažuriranje. To se događa transparentno bez potrebe za intervencijom korisnika.

### Razmatranja propusnosti

WebSocket poruke su lagane i sadrže samo bitne informacije potrebne za sinkronizaciju stanja. Stvaranje novog markera obično koristi manje od 1KB propusnosti. Sustav također uključuje inteligentno grupiranje kako bi se smanjila učestalost poruka tijekom razdoblja visoke aktivnosti.

Vaše metrike korištenja u FastComments nadzornoj ploči prate `pubSubMessageCount` i `pubSubBandwidth` kako biste mogli pratiti aktivnost sinkronizacije u stvarnom vremenu na svojim stranicama.

### Sinkronizacija između kartica

Ako korisnik ima istu stranicu otvorenu u više kartica preglednika, ažuriranja u jednoj kartici odmah se pojavljuju u ostalim karticama. To funkcionira kroz isti mehanizam WebSocket sinkronizacije i ne zahtijeva nikakvu dodatnu konfiguraciju.

Korisnici mogu imati vašu stranicu otvorenu na više uređaja istovremeno, i svi će ostati sinkronizirani. Marker stvoren na računalu odmah se pojavljuje na korisnikovom tabletu ako oba uređaja gledaju istu sliku.

### Sigurnost

WebSocket poruke se prenose preko sigurnih veza (WSS) i uključuju validaciju stanara (tenant) kako bi se osiguralo da korisnici primaju samo ažuriranja za razgovore koje su ovlašteni vidjeti. Poslužitelj validira sve operacije prije nego što ih emitira kako bi spriječio neovlašteni pristup ili manipulaciju.

### Ponašanje izvan mreže

Kada su korisnici potpuno izvan mreže, i dalje mogu vidjeti postojeće markere, ali ne mogu stvarati nove niti vidjeti ažuriranja drugih. Widget otkriva stanje izvan mreže i prikazuje odgovarajuću poruku.

Ako korisnik pokuša stvoriti marker dok je izvan mreže, a zatim se vrati na mrežu, operacija će propasti umjesto da se stavi u red čekanja, čime se osigurava konzistentnost podataka. Korisnici trebaju ponoviti operaciju nakon što se veza obnovi.

### Utjecaj na performanse

WebSocket veza ima minimalan utjecaj na performanse. Veza ostaje neaktivna kada nema ažuriranja i obrađuje poruke samo kada se dogodi aktivnost. Na tipičnoj slici s umjerenom aktivnošću markera, WebSocket koristi manje CPU-a nego samo renderiranje slike.

Za stranice sa stotinama istovremenih korisnika i visokom aktivnošću stvaranja markera, sustav se horizontalno skalira kako bi održao performanse bez utjecaja na pojedinačne klijentske veze.

### Suradnički primjeri upotrebe

Sinkronizacija u stvarnom vremenu čini Image Chat posebno moćnim za suradničke tijekove rada. Dizajnerski timovi mogu zajedno pregledavati mockupove dok svi u stvarnom vremenu vide postavljanje markera. Timovi za korisničku podršku mogu suradnički označavati snimke zaslona kako bi identificirali probleme. Obrazovne skupine mogu raspravljati o dijagramima dok svi sudionici vide markere jedni drugih čim se stvore.

Trenutna povratna informacija stvara angažiranije i produktivnije suradničko iskustvo u usporedbi s tradicionalnim sustavima komentara gdje korisnici trebaju osvježiti stranicu da vide ažuriranja.