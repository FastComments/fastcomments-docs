FastComments podržava automatski način održavanja. Ako baza podataka padne, može nastaviti služiti popularne niti komentara.

Osim toga, u načinu održavanja svi se komentari spremaju u `BACKUP_DIR`. Oni će biti obrađeni (provjereni na spam itd.) i spremljeni kad sustav ponovno bude online.

To radi tako da svakog sata određuje 100 najpopularnijih niti komentara i kešira njihov sadržaj na disku. Određivanje top 100 niti
već se radi iz unaprijed izračunatog stanja, tako da nije težak periodični zadatak.

Ovo je potpuno opcionalno i omogućeno je samo ako su postavljeni `CACHE_DIR` i `BACKUP_DIR`. To naravno čini čvorove aplikacije ovisnima o stanju, no to je stanje koje se može izgubiti u bilo kojem trenutku bez da uzrokuje pogrešno ponašanje aplikacije.

Imajte na umu da se u načinu održavanja ne može provesti ispravna autentikacija niti komentara, pa se povremeno spremaju samo niti koje se sigurno smatraju javnima.

U načinu održavanja mnoge značajke nisu dostupne.