Okida kada se komentar obriše.

### Kontekst koji agent prima

- Komentar koji je upravo obrisan - tekst, autor, stranica.
- Opcionalni kontekst teme / historija korisnika / kontekst stranice prema konfiguraciji.

### Napomene

- Okida se i za **soft deletes** (gdje je komentar skriven ali zadržan radi revizije) i za **hard deletes** (gdje je komentar u potpunosti uklonjen). Rukovalac okidača razrješava komentar iz pipeline-a kaskadnog brisanja; ono što agent vidi je posljednje poznato stanje.
- Kada je komentar u potpunosti obrisan, alati koji ga ciljaju (`pin_comment`, `mark_comment_spam`, itd.) koristeći taj ID komentara neće uspjeti.

### Uobičajene upotrebe

- **Prosljeđivanje revizije putem [Webhooks](#webhooks-overview)** - pošalje `trigger.succeeded` događaj kako bi eksterni sistem zabilježio šta je obrisano.
- **Upisi u memoriju** - neka agent zabilježi [bilješku u memoriji](#tools-overview) o obrascu brisanja (obrisani komentar je bio treći komentar korisnika u 24 sata, itd.).
- **Efekti između tema** - primijeti kada brisanje promijeni strukturu teme koju je agent ranije sažeo, i razmotri da li treba ponovo sažeti.

### Napomena o troškovima rada

Ako imate sajt sa velikim obimom brisanja (intenzivna ljudska moderacija), ovaj okidač se može često aktivirati.