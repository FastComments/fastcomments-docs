## Rešavanje problema

**"Nemate dozvolu" prilikom povezivanja.** Ulogovani korisnik nije API administrator na nalogu.  
Zamolite vlasnika naloga da dodeli API dozvolu na stranici Korisnici, ili se povežite kao vlasnik.

**Veza je označena pogrešnim sajtom.** Stranica za saglasnost povezuje nalog na koji ste bili ulogovani u tom trenutku.  
Prekinite vezu u Zapieru, promenite nalog u FastComments kontrolnoj tabli, i ponovo se povežite.

**Događaji su prestali da stižu.** Proverite stranicu Webhook-ova u kontrolnoj tabli.  
Pretplata čiji je krajnji URL neuspela šest dana automatski se onemogućava i prikazuje razlog. Ponovo je omogućite tamo, ili isključite i ponovo uključite Zap.  
Ako pretplata u potpunosti nedostaje, neko ju je obrisao; isključivanje i ponovo uključivanje Zapa je ponovo kreira.

**Zapier kaže da je potrebno ponovo povezati nalog.** Veza je povučena sa stranice Povezane aplikacije, korisnik koji ju je odobrio je izgubio API dozvolu, ili je nalog obrisan. Ponovo se povežite iz Zapiera.

**Akcija ne uspeva sa porukom "nema pristup za pisanje".** Veza je odobrena samo sa dozvolom za čitanje. Ponovo se povežite i odobrite obe dozvole.

**Ograničenja brzine i krediti.** Akcije i pretrage troše API kredite iz vašeg plana i podložne su istim ograničenjima brzine kao REST API. Okidači ne troše ništa. Zap koji dosegne limit se ponovo pokušava od strane Zapiera nakon kašnjenja koje FastComments izvešta.

**Padajući meni Domen je prazan.** Domeni se pojavljuju kada su konfigurisani na stranici Domeni u FastComments kontrolnoj tabli. Ostavite polje prazno da primate događaje za svaki domen.