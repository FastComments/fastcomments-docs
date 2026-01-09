---
### Zahtevane komponente

Za On-Prem je FastComments sestavljen le iz aplikacijskega strežnika in baze podatkov. Poenostavili smo namestitev, tako da aplikacija lahko neposredno obdeluje ves promet brez dodajanja drugih komponent.

Aplikacijski strežnik je na voljo v Docker sliki in ga je mogoče namestiti z poljubno rešitvijo za upravljanje kontejnerjev.

Bazo podatkov, MongoDB, lahko gostite sami ali pa jo upravlja drug ponudnik, kot sta AWS DocumentDB ali MongoDB Atlas.

FastComments je trenutno preizkušen z MongoDB 7, vendar si prizadevamo za združljivost z DocumentDB, da poenostavimo namestitev.

### Velikosti instanc

Ugotovili boste, da je FastComments dokaj optimiziran in za samo aplikacijo ne zahteva velikih strojev, da ohrani nizke P99s.

Vsa batch in cron opravila uporabljajo streaming, da omejijo skupno rabo pomnilnika.

Spodnje tabele za aplikacijski strežnik in bazo podatkov lahko pomagajo pri določanju velikosti.

### Instance aplikacijskega strežnika


| Sočasni uporabniki | Skupni CPU-ji gruče | Skupni pomnilnik gruče |
|--------------------|---------------------|------------------------|
| 100                | 1                   | 256mb                 |
| 1K                 | 2                   | 512mb                 |
| 10K                | 8                   | 1gb                   |
| 100K               | 32                  | 8gb                   |
| 1M                 | 64                  | 64gb                  |

Na primer, enojno jedro, ki obdela približno 100 niti komentarjev na sekundo, ponavadi ne porabi več kot 250mb RSS.

### Instance strežnika baze podatkov

Določanje velikosti baze podatkov je odvisno od velikosti delovnega nabora (working set), kar je količina podatkov, do katerih dostopate v določenem trenutku, pa tudi od sočasnih zahtev.

FastComments je Mongo prijazen, saj za pogoste poizvedbe uporablja index hints, streaming cursors in ima omejitve sočasnosti na različnih področjih, da prepreči preobremenitev nadaljnjih sistemov.

Spodaj je splošno vodilo za velikosti instanc baze podatkov. **Upoštevajte, da je to __na instanco__, ne skupni viri v gruči**.

| Sočasni uporabniki | Shranjeni komentarji | CPU-ji na instanco | Pomnilnik na instanco |
|--------------------|----------------------|--------------------|-----------------------|
| 100                | 1k                   | 1                  | 256mb                |
| 1K                 | 5k                   | 2                  | 512mb                |
| 10K                | 100k                 | 8                  | 2gb                  |
| 100K               | 500k                 | 16                 | 8gb                  |
| 1M                 | 5M                   | 32                 | 32gb                 |

Zgornje tabele so konservativne ocene. Dejanske zahteve se lahko razlikujejo glede na vašo specifično konfiguracijo (velikost strani, volumen komentarjev itd.).
---