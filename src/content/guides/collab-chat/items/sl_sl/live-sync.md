### Posodobitve v realnem času

Collab Chat uporablja WebSocket povezave za sinhronizacijo vseh pogovorov v realnem času med vsemi povezanimi uporabniki. Ko nekdo ustvari novo anotacijo, doda komentar ali izbriše razpravo, vsi drugi uporabniki, ki si ogledujejo isto stran, vidijo posodobitev takoj brez osvežitve.

### Kako deluje sinhronizacija WebSocket

Ko inicializirate Collab Chat, pripomoček vzpostavi WebSocket povezavo s strežniki FastComments. Ta povezava ostane odprta za čas trajanja uporabnikove seje in posluša posodobitve, povezane s trenutno stranjo.

Sistem WebSocket uporablja tri vrste broadcast sporočil za Collab Chat. Dogodek `new-text-chat` se sproži, ko nekdo ustvari novo anotacijo na strani. Dogodek `updated-text-chat` se sproži, ko nekdo posodobi obstoječi pogovor. Dogodek `deleted-text-chat` se sproži, ko nekdo izbriše anotacijo.

### Sistem Broadcast ID

Da preprečimo učinke odmeva, pri katerih uporabniki vidijo svoje lastne ukrepe, ki so jim ponovno oddani, vsaka posodobitev vključuje edinstveni `broadcastId`. Ko uporabnik ustvari ali posodobi anotacijo, njegov odjemalec ustvari UUID za to operacijo. Ko WebSocket odda posodobitev nazaj vsem odjemalcem, izvorni odjemalec zanemari posodobitev, ker se ujema z njegovim `broadcastId`.

To zagotavlja tekočo interakcijo, kjer uporabniki takoj vidijo svoje spremembe v uporabniškem vmesniku brez čakanja na krožno pot prek strežnika, hkrati pa se zagotovi, da vsi ostali uporabniki prejmejo posodobitev.

### Število uporabnikov v živo

V zgornji vrstici je prikazano število uporabnikov, ki si trenutno ogledujejo stran. To število se posodablja v realnem času, ko se uporabniki pridružujejo in odhajajo. Število uporabnikov je zagotovljeno prek iste WebSocket povezave in se samodejno poveča/ zmanjša glede na dogodke vzpostavitve in prekinitev povezave.

### Odpornost povezave

Če WebSocket povezava pade zaradi težav v omrežju ali vzdrževanja strežnika, pripomoček samodejno poskusi znova vzpostaviti povezavo. Med obdobjem ponovnega vzpostavljanja lahko uporabniki še vedno sodelujejo z obstoječimi anotacijami, vendar ne bodo videli posodobitev v realnem času od drugih uporabnikov, dokler povezava ne bo ponovno vzpostavljena.

Ko je povezava znova vzpostavljena, se pripomoček ponovno sinhronizira, da zagotovi, da nobena posodobitev ni bila spregledana. To se zgodi samodejno, brez potrebe po posegu uporabnika.

### Premisleki glede pasovne širine

WebSocket sporočila so lahka in vsebujejo le bistvene informacije, potrebne za sinhronizacijo stanja. Ustvarjanje nove anotacije običajno porabi manj kot 1KB pasovne širine. Sistem vključuje tudi inteligentno združevanje sporočil, da zmanjša frekvenco sporočil med obdobji velike aktivnosti.

Vaše meritve uporabe na nadzorni plošči FastComments spremljajo `pubSubMessageCount` in `pubSubBandwidth`, tako da lahko spremljate dejavnost sinhronizacije v realnem času na vaših spletnih mestih.

### Sinhronizacija med zavihki

Če ima uporabnik isto stran odprto v več zavihkih brskalnika, se posodobitve v enem zavihku takoj prikažejo v drugih zavihkih. To deluje prek istega mehanizma WebSocket sinhronizacije in ne zahteva dodatne konfiguracije.

### Varnost

WebSocket sporočila se prenašajo prek varnih povezav (WSS) in vključujejo preverjanje najemnika, da se zagotovi, da uporabniki prejemajo posodobitve le za pogovore, za katere so pooblaščeni. Strežnik preveri vse operacije, preden jih odda, da prepreči nepooblaščen dostop ali manipulacijo.