## Rješavanje problema

**"Nemate dozvolu" prilikom povezivanja.** Prijavljeni korisnik nije API administrator na računu.  
Zatražite od vlasnika računa da odobri API dozvolu na stranici Korisnici, ili se povežite kao vlasnik.

**Veza je označena s pogrešnom stranicom.** Stranica za odobrenje povezuje račun na koji ste bili prijavljeni  
u tom trenutku. Prekinite vezu u Zapieru, promijenite račun u FastComments nadzornoj ploči i ponovno se povežite.

**Događaji su prestali dolaziti.** Provjerite stranicu Webhookova u nadzornoj ploči. Pretplata čiji je krajnji punkt neuspješno radila šest dana automatski se onemogućuje i prikazuje razlog. Ponovno je omogućite tamo, ili isključite i ponovno uključite Zap. Ako pretplata uopće nedostaje, netko ju je izbrisao; isključivanje i ponovno uključivanje Zapa će je ponovno stvoriti.

**Zapier kaže da je račun potrebno ponovno povezati.** Veza je povučena s stranice Povezane aplikacije, korisnik koji ju je odobrio izgubio je API dozvolu, ili je račun izbrisan. Ponovno se povežite iz Zapia.

**Radnja ne uspijeva s porukom "nema pristup za pisanje".** Veza je odobrena s dozvolom samo za čitanje. Ponovno se povežite i odobrite obje dozvole.

**Ograničenja brzine i krediti.** Radnje i pretrage troše API kredite iz vašeg plana i podliježu istim ograničenjima brzine kao REST API. Okidači ne troše ništa. Zap koji dosegne ograničenje ponovno se pokušava od strane Zapia nakon kašnjenja koje FastComments prijavljuje.

**Padajući izbornik Domene je prazan.** Domene se pojavljuju nakon što su konfigurirane na stranici Domene u FastComments nadzornoj ploči. Ostavite polje prazno da primate događaje za svaku domenu.