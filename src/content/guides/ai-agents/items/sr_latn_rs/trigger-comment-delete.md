Okida se kada se komentar obriše.

### Kontekst koji agent prima

- Komentar koji je upravo obrisan - tekst, autor, stranica.
- Opcioni kontekst teme / istorije korisnika / stranice kako je konfigurisano.

### Napomene

- Okida se i za **soft deletes** (gde je komentar sakriven ali zadržan za audit) i za **hard deletes** (gde je komentar potpuno uklonjen). Handler okidača rešava komentar iz cascade delete pipeline; ono što agent vidi je poslednje poznato stanje.
- Kada je komentar u potpunosti obrisan, alati koji ciljaju na njega (`pin_comment`, `mark_comment_spam`, itd.) za taj ID komentara biće neuspešni.

### Uobičajene upotrebe

- **Prosleđivanje revizije putem [Webhooks](#webhooks-overview)** - emituje `trigger.succeeded` događaj tako da eksterni sistem zabeleži šta je obrisano.
- **Upis u memoriju** - neka agent zabeleži [belešku u memoriji](#tools-overview) o obrascu brisanja (obrisani komentar je bio treći korisnikov u 24 sata, itd.).
- **Efekti između tema** - primećuje kada brisanje menja strukturu teme koju je agent prethodno sumirao, i razmotrite da li treba ponovo sažeti.

### Napomena o operativnim troškovima

Ako imate sajt sa velikim obimom brisanja (intenzivna ljudska moderacija), ovaj okidač se može često pokretati.