### Potrebne komponente

Za On-Prem, FastComments se sastoji samo od aplikacijskog poslužitelja i baze podataka. Pojednostavili smo implementaciju tako da
aplikacija može opslužiti sav promet izravno bez dodavanja drugih komponenti.

Aplikacijski poslužitelj dostavljen je kao Docker image i može se implementirati s bilo kojim rješenjem za upravljanje kontejnerima.

Bazu podataka, MongoDB, možete pokrenuti sami ili je može hostirati drugi pružatelj usluga poput AWS DocumentDB ili MongoDB Atlas.

FastComments je trenutno testiran s MongoDB 7, međutim težimo kompatibilnosti s DocumentDB kako bismo olakšali implementaciju.

### Veličine instanci

Otkrit ćete da je FastComments prilično dobro optimiziran i ne zahtijeva velike strojeve za samu aplikaciju kako bi održao niske P99 vrijednosti.

Sve batch i cron zadatke koriste streaming kako bi ograničili ukupnu potrošnju memorije.

Donje tablice za aplikacijski poslužitelj i bazu podataka mogu pomoći pri određivanju veličine.

### Instance aplikacijskog poslužitelja


| Istovremeni korisnici | Ukupno CPU-a klastera | Ukupna memorija klastera |
|------------------|--------------------|----------------------|
| 100              | 1                  | 256mb                |
| 1K               | 2                  | 512mb                |
| 10K              | 8                  | 1gb                  |
| 100K             | 32                 | 8gb                  |
| 1M               | 64                 | 64gb                 |

Na primjer, jedno jezgro koje opslužuje oko 100 niti komentara u sekundi obično nikada ne koristi više od 250mb RSS-a.

### Instance poslužitelja baze podataka

Veličina baze podataka ovisi o veličini radnog skupa, tj. o količini podataka kojima pristupate u određenom trenutku, kao i o istovremenim zahtjevima.

FastComments je prilično blagonaklon prema Mongodb-u, u smislu da za vruće upite koristi index hints, streaming cursors, te ima ograničenja konkurentnosti u raznim područjima
kako bi spriječio preopterećenje sustava nizvodno.

Donje smjernice su općenite za veličine instanci baze podataka. **Imajte na umu da se ovo odnosi na __po instanci__, a ne na ukupne resurse u klasteru**.

| Istovremeni korisnici | Pohranjeni komentari | CPU-a po instanci | Memorija po instanci |
|------------------|-----------------|-------------------|---------------------|
| 100              | 1k              | 1                 | 256mb               |
| 1K               | 5k              | 2                 | 512mb               |
| 10K              | 100k            | 8                 | 2gb                 |
| 100K             | 500k            | 16                | 8gb                 |
| 1M               | 5M              | 32                | 32gb                |

Gornje tablice su konzervativne procjene. Možete otkriti da se stvarni zahtjevi razlikuju ovisno o vašoj specifičnoj konfiguraciji (veličine stranica, obujam komentara itd).