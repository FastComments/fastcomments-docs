FastComments podržava automatski režim održavanja. Ako baza podataka padne, može nastaviti da servisira popularne niti komentara.

Pored toga, u režimu održavanja svi komentari se čuvaju u `BACKUP_DIR`. Oni će biti obrađeni (provereni na spam, itd.) i sačuvani kada sistem ponovo bude online.

To radi tako što, svakog sata, određuje 100 najpopularnijih niti komentara i kešira njihov sadržaj na disku. Određivanje 100 najpopularnijih niti
već se radi iz prethodno izračunatog stanja, tako da nije zahtevan periodični posao.

Ovo je potpuno opciono, i omogućeno je samo ako su postavljeni `CACHE_DIR` i `BACKUP_DIR`. Ovo naravno čini čvorove aplikacije sa stanjem, međutim to je stanje koje
može biti izgubljeno u bilo kom trenutku bez izazivanja nepravilnog rada aplikacije.

Imajte na umu da u režimu održavanja pravilna autentifikacija niti komentara ne može biti izvršena, pa se periodično prave rezervne kopije samo niti koje se bezbedno smatraju javnim.

U režimu održavanja mnoge funkcije nisu dostupne.