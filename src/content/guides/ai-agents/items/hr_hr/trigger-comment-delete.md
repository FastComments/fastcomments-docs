Pokreće se kada je komentar izbrisan.

### Kontekst koji agent prima

- Komentar koji je upravo izbrisan - tekst, autor, stranica.
- Opcionalni kontekst teme / povijest korisnika / kontekst stranice kako je konfigurirano.

### Važno

- Pokreće se i za **privremeno brisanje** (gdje je komentar skriven, ali zadržan radi revizije) i za **trajno brisanje** (gdje je komentar potpuno uklonjen). Hander okidača rješava komentar iz toka kaskadnog brisanja; ono što agent vidi je posljednje poznato stanje.
- Kad je komentar potpuno izbrisan, alati koji ciljaju na njega (`pin_comment`, `mark_comment_spam`, itd.) s tom ID-om komentara neće uspjeti.

### Uobičajene upotrebe

- **Prosljeđivanje revizije putem [Webhooks](#webhooks-overview)** - emitirajte događaj `trigger.succeeded` kako bi vanjski sustav zabilježio što je izbrisano.
- **Zapisivanje u memoriju** - neka agent zabilježi [bilješku u memoriji](#tools-overview) o obrascu brisanja (izbrisani komentar bio je treći korisnikov u 24 sata, itd.).
- **Učinak na druge teme** - primijetite kada brisanje mijenja strukturu teme koju je agent prethodno sažeo i razmotrite treba li ponovno sažeti.

### Napomena o trošku rada

Ako imate stranicu s velikim brojem brisanja (intenzivna ljudska moderacija), ovaj okidač može se često pokretati.

---