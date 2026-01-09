FastComments podpira samodejni način vzdrževanja. Če baza podatkov preneha delovati, lahko še naprej streže priljubljene niti komentarjev.

Poleg tega se v načinu vzdrževanja vsi komentarji shranijo v `BACKUP_DIR`. Obdelani bodo (preverjeni glede neželene pošte itd.) in shranjeni, ko bo sistem spet na voljo.

To stori tako, da vsako uro določi 100 najbolj priljubljenih nitk komentarjev in njihove vsebine predpomni na disku. Določanje top 100 nitk je že izvedeno na podlagi vnaprej izračunanega stanja, zato ni zahtevna periodična naloga.

To je popolnoma izbirno in je omogočeno samo, če sta nastavljena `CACHE_DIR` in `BACKUP_DIR`. Seveda to naredi vozlišča aplikacije z ohranjanjem stanja, vendar gre za stanje, ki se lahko kadarkoli izgubi, ne da bi to povzročilo nepravilno delovanje aplikacije.

Upoštevajte, da v načinu vzdrževanja ni mogoče pravilno preveriti pristnosti niti komentarjev, zato se periodično varnostno kopirajo le niti, ki jih varno štejemo za javne.

V načinu vzdrževanja mnoge funkcije niso na voljo.