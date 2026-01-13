### Posodobitve v realnem času

Image Chat uporablja WebSocket povezave za sinhronizacijo vseh pogovorov v realnem času med vsemi povezanimi uporabniki. Ko nekdo ustvari nov marker, doda komentar ali izbriše razpravo, vsi drugi uporabniki, ki gledajo isto sliko, vidijo posodobitev takoj brez osvežitve.

### Kako deluje WebSocket sinhronizacija

Ko inicializirate Image Chat, pripomoček vzpostavi WebSocket povezavo s strežniki FastComments. Ta povezava ostane odprta za trajanje uporabnikove seje in posluša posodobitve, povezane z trenutno sliko.

WebSocket sistem uporablja tri vrste oddajnih sporočil za Image Chat. Dogodek `new-image-chat` se sproži, ko nekdo ustvari nov marker na sliki. Dogodek `image-chat-updated` se sproži, ko nekdo posodobi obstoječ pogovor. Dogodek `deleted-image-chat` se sproži, ko nekdo izbriše marker.

### Sistem Broadcast ID

Da bi preprečili učinek odmeva, kjer uporabniki vidijo svoje lastne akcije, poslane nazaj, vsaka posodobitev vključuje edinstven `broadcastId`. Ko uporabnik ustvari ali posodobi marker, njegov odjemalec ustvari UUID za to operacijo. Ko WebSocket odda posodobitev nazaj vsem odjemalcem, izvorni odjemalec to posodobitev ignorira, ker se ujema z njegovim `broadcastId`.

To zagotavlja gladko interakcijo, kjer uporabniki vidijo svoje spremembe takoj v uporabniškem vmesniku brez čakanja na povratno pot do strežnika, medtem ko še vedno zagotavlja, da vsi drugi uporabniki prejmejo posodobitev.

### Odpornost povezave

Če WebSocket povezava pade zaradi težav z omrežjem ali vzdrževanja strežnika, pripomoček samodejno poskusi znova vzpostaviti povezavo. Med obdobjem ponovne povezave lahko uporabniki še vedno sodelujejo z obstoječimi markerji, vendar ne bodo videli posodobitev v realnem času od drugih uporabnikov, dokler povezava ni ponovno vzpostavljena.

Ko je povezava ponovno vzpostavljena, se pripomoček ponovno sinhronizira, da se zagotovi, da ni bilo zamujenih posodobitev. To se zgodi transparentno, brez potrebe po posegu uporabnika.

### Premisleki glede pasovne širine

WebSocket sporočila so lahka in vsebujejo le bistvene informacije, potrebne za sinhronizacijo stanja. Ustvarjanje novega markerja običajno porabi manj kot 1KB pasovne širine. Sistem vključuje tudi inteligentno združevanje sporočil, da zmanjša pogostost pošiljanja v obdobjih velike aktivnosti.

Vaše meritve uporabe v nadzorni plošči FastComments spremljajo `pubSubMessageCount` in `pubSubBandwidth`, tako da lahko spremljate dejavnost sinhronizacije v realnem času na vaših spletnih mestih.

### Sinhronizacija med zavihki

Če ima uporabnik isto stran odprto v več zavihkih brskalnika, se posodobitve v enem zavihku takoj pojavijo v drugih zavihkih. To deluje preko istega mehanizma WebSocket sinhronizacije in ne zahteva dodatne konfiguracije.

Uporabniki lahko imajo vašo stran odprto na več napravah hkrati in vse bodo ostale sinhronizirane. Marker, ustvarjen na namiznem računalniku, se takoj pojavi na uporabnikovi tablici, če obe napravi gledata isto sliko.

### Varnost

WebSocket sporočila se prenašajo preko varnih povezav (WSS) in vključujejo preverjanje najemnika (tenant validation), da se zagotovi, da uporabniki prejemajo le posodobitve za pogovore, do katerih so pooblaščeni. Strežnik preveri vse operacije pred njihovim oddajanjem, da prepreči nepooblaščen dostop ali manipulacijo.

### Obnašanje brez povezave

Ko so uporabniki popolnoma brez povezave, lahko še vedno ogledajo obstoječe markerje, vendar ne morejo ustvarjati novih ali videti posodobitev drugih. Pripomoček zazna stanje brez povezave in prikaže ustrezno sporočilo.

Če uporabnik poskusi ustvariti marker, medtem ko je brez povezave, in se nato znova poveže, bo operacija neuspešna namesto da bi se postavila v čakalno vrsto, s čimer se zagotovi konsistentnost podatkov. Uporabniki naj poskusijo znova, ko je njihova povezava obnovljena.

### Vpliv na zmogljivost

Povezava WebSocket ima minimalen vpliv na zmogljivost. Povezava je neaktivna, kadar ne potekajo posodobitve, in obdeluje sporočila le, ko se pojavi aktivnost. Na tipični sliki z zmerno aktivnostjo markerjev WebSocket porabi manj CPU kot samo upodabljanje slike.

Za strani s stovkami sočasnih uporabnikov in visoko aktivnostjo ustvarjanja markerjev se sistem horizontalno razširi, da ohrani zmogljivost, ne da bi vplival na posamezne odjemalske povezave.

### Sodelovalni primeri uporabe

Sinhronizacija v realnem času naredi Image Chat še posebej močan za sodelovalne delovne tokove. Oblikovalne ekipe lahko skupaj pregledujejo makete in vsi vidijo pozicioniranje markerjev v realnem času. Ekipe za podporo strankam lahko skupaj označujejo posnetke zaslona, da prepoznajo težave. Izobraževalne skupine lahko razpravljajo o diagramih, pri čemer vsi udeleženci vidijo markerje drugih, ko so ustvarjeni.

Takojšnja povratna informacija ustvarja bolj privlačno in produktivno sodelovalno izkušnjo v primerjavi s tradicionalnimi sistemi komentarjev, kjer morajo uporabniki osvežiti stran, da vidijo posodobitve.