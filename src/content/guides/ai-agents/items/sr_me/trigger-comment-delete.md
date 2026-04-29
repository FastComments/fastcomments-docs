Aktivira se kada je komentar obrisan.

### Kontekst koji agent prima

- Komentar koji je upravo obrisan - tekst, autor, stranica.
- Opcioni kontekst niti / istorija korisnika / kontekst stranice kako je konfigurirano.

### Napomena

- Aktivira se za oba slučaja **mekog brisanja** (gde je komentar skriven ali zadržan radi revizije) i **potpunog brisanja** (gde je komentar u potpunosti uklonjen). Handler okidača rešava komentar iz pipeline-a kaskadnog brisanja; ono što agent vidi je poslednje poznato stanje.
- Kada je komentar potpuno obrisan, alati koji ciljaju taj ID komentara (`pin_comment`, `mark_comment_spam`, itd.) neće uspeti.

### Uobičajena upotreba

- **Prosleđivanje revizije putem [Webhooks](#webhooks-overview)** - emituje događaj `trigger.succeeded` kako bi eksterni sistem zabeležio šta je obrisano.
- **Upisi u memoriju** - omogućiti agentu da zabeleži [memorijsku belešku](#tools-overview) o obrascu brisanja (obrisani komentar je bio korisnikov treći u roku od 24 sata, itd.).
- **Efekti na više niti** - primetiti kada brisanje menja strukturu niti koju je agent prethodno sažimao, i razmotriti da li treba ponovo sažeti.

### Napomena o troškovima rada

Ako imate sajt sa velikim obimom brisanja (intenzivna ljudska moderacija), ovaj okidač se može često aktivirati.