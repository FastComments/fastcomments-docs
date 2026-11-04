### Potrebne komponente

Za On-Prem, FastComments se sastoji samo od aplikacionog servera i baze podataka. Pojednostavili smo implementaciju tako da
aplikacija može direktno opslužiti sav saobraćaj bez dodavanja drugih komponenti.

Aplikacioni server se isporučuje kao Docker slika i može se implementirati sa bilo kojim rešenjem za upravljanje kontejnerima.

Baza podataka, MongoDB, može biti samostalno pokrenuta ili hostovana od strane drugog provajdera kao što su AWS DocumentDB ili MongoDB Atlas.

FastComments je trenutno testiran sa MongoDB 7, međutim težimo ka DocumentDB kompatibilnosti radi lakše implementacije.

### Veličine instanci

Uvidećete da je FastComments prilično dobro optimizovan i ne zahteva velike mašine za samu aplikaciju da bi održao niske P99 vrednosti.

Svi batch i cron poslovi koriste streaming kako bi ograničili ukupnu potrošnju memorije.

Tabele ispod za aplikacioni server i bazu podataka mogu pomoći pri određivanju veličine.

### Instance aplikacionog servera


| Istovremeni korisnici | Ukupno CPU u klasteru | Ukupna memorija klastera |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

Na primer, jedno jezgro koje opslužuje oko 100 niti komentara u sekundi obično nikada ne koristi više od 250mb RSS.

### Instance servera baze podataka

Veličina baze podataka zavisi od veličine radnog skupa, odnosno količine podataka kojima pristupate u datom trenutku, kao i od istovremenih zahteva.

FastComments je prilično blag prema Mongo-u, u tome što za najčešće upite koristi index hints, streaming cursors, i ima ograničenja konkurentnosti u raznim oblastima
kako bi sprečio preopterećenje downstream sistema.

Ispod je opšta smernica za veličine instanci baze podataka. **Imajte na umu da je ovo __po instanci__, a ne ukupni resursi u klasteru**.

| Istovremeni korisnici | Sačuvanih komentara | CPU po instanci | Memorija po instanci |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

Gornje tabele su konzervativne procene. Možete otkriti da se stvarni zahtevi razlikuju u zavisnosti od vaše specifične konfiguracije (veličine stranica, obim komentara, itd).