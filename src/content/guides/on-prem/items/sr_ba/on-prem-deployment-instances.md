### Potrebne Komponente

Za On-Prem, FastComments se sastoji samo od aplikacionog servera i baze podataka. Pojednostavili smo deployment tako da
aplikacija može direktno servisirati sav promet bez dodavanja drugih komponenti.

Aplikacioni server je dostavljen u Docker image-u i može se deploy-ati sa bilo kojim rješenjem za upravljanje kontejnerima.

Baza podataka, MongoDB, može se pokretati samostalno ili hostovati kod drugog provajdera kao što su AWS DocumentDB ili MongoDB Atlas.

FastComments je trenutno testiran sa MongoDB 7, međutim cilj nam je da bude kompatibilan sa DocumentDB radi lakšeg deploy-ovanja.

### Veličine instanci

Uvidjet ćete da je FastComments prilično dobro optimizovan i da za samu aplikaciju nisu potrebne velike mašine kako bi se održali niski P99s.

Svi batch i cron poslovi koriste streaming da ograniče ukupnu upotrebu memorije.

Ispod tabele za aplikacioni server i bazu podataka mogu pomoći pri određivanju veličine.

### Instance aplikacionog servera


| Istovremeni korisnici | Ukupan broj CPU-a klastera | Ukupna memorija klastera |
|-----------------------|----------------------------|--------------------------|
| 100                   | 1                          | 256mb                   |
| 1K                    | 2                          | 512mb                   |
| 10K                   | 8                          | 1gb                     |
| 100K                  | 32                         | 8gb                     |
| 1M                    | 64                         | 64gb                    |

Na primjer, jedna jezgra koja servisira oko 100 niti komentara u sekundi obično nikad ne koristi više od 250mb RSS.

### Instance servera baze podataka

Dimenzionisanje baze podataka zavisi od veličine working seta, što je količina podataka kojoj pristupate u određenom trenutku, kao i od istovremenih zahtjeva.

FastComments je prilično blagonaklon prema Mongo-u, te za "hot" upite koristi index hints, streaming cursors, i ima ograničenja konkurentnosti u raznim oblastima
da spreči preopterećenje downstream sistema.

Ispod je opšti vodič za veličine instanci baze podataka. **Napomena: ovo je __po instanci__, ne ukupni resursi u klasteru**.

| Istovremeni korisnici | Spremljeni komentari | CPU-a po instanci | Memorija po instanci |
|-----------------------|----------------------|-------------------|----------------------|
| 100                   | 1k                   | 1                 | 256mb               |
| 1K                    | 5k                   | 2                 | 512mb               |
| 10K                   | 100k                 | 8                 | 2gb                 |
| 100K                  | 500k                 | 16                | 8gb                 |
| 1M                    | 5M                   | 32                | 32gb                |

Gornje tabele su konzervativne procjene. Možete utvrditi da se stvarni zahtjevi razlikuju u zavisnosti od vaše specifične konfiguracije (veličine stranica, volumen komentara, itd).