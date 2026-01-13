Po zadanim postavkama, FastComments dolazi s trenirajućim sustavom za otkrivanje neželjene pošte.

Kako moderirate komentare i označavate ih kao **Spam**, ili označavate komentare koje je sustav automatski pronašao kao **Spam** kao **Not Spam**, sustav za otkrivanje neželjene pošte uči iz tih radnji kako bi točnije odredio što želite smatrati spamanjem.

Komentari označeni kao **Spam** neće biti automatski odobreni, pa se neće prikazivati dok ih eksplicitno ne označite kao **Not Spam**.

Otkrivanje spama može se onemogućiti putem stranice Postavke moderiranja komentara.

### Different Spam Detectors

FastComments podržava tri načina otkrivanja spama:

1. A traditional Naïve-Bayes classifier that is continuously trained, which is shared across all FastComments.com tenants.
2. A traditional Naïve-Bayes classifier that is continuously trained, which is **isolated** to your tenant.
3. Using ChatGPT 4.

Svi imaju pristup shared i isolated Naïve-Bayes klasifikatorima.

Opcija ChatGPT 4 može se odabrati na stranici Postavke moderiranja komentara ako ste na Flex billing planu, budući da se naplaćuje na temelju iskorištenih tokena.

### Faktor povjerenja

FastComments prilagođava filtar za spam za korisnika na temelju toga koliko mu se vjeruje na toj stranici.

Na primjer, ako su administratori puno puta prikvačili njihove komentare, vjerojatno je da je riječ o vrlo pouzdanom korisniku. Ili, ako su dugo član stranice i imaju mnogo komentara, njihov faktor povjerenja također može biti visok.

### SSO

Komentari koje objavljuju SSO korisnici mogu se smatrati spamom i bit će provjereni kao takvi. Izuzetak je ako SSO korisnik ima isti email kao tenant korisnik koji ima jednu ili više sljedećih dozvola:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO korisnici s tim dozvolama neće imati svoje komentare provjeravane na spam.

### Ponavljane poruke

FastComments će otkriti i spriječiti ponovljene poruke. Također će otkriti ponovljene poruke koje su vrlo slične kako bi pomogao spriječiti spam. Ovo se ne može onemogućiti jer sprječava zloupotrebu naše platforme. Ako imate visok faktor povjerenja, to se uzima u obzir prilikom sprječavanja ponovljenih poruka.