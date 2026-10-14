Po zadanim postavkama, FastComments dolazi s trenirajućim sustavom za otkrivanje neželjene pošte.

Kako moderirate komentare i označavate ih kao **Spam**, ili označavate komentare koje je sustav automatski pronašao kao **Spam** kao **Nije spam**, sustav za otkrivanje neželjene pošte uči iz tih radnji kako bi točnije odredio što želite smatrati spamom.

Komentari označeni kao **Spam** neće biti automatski odobreni, pa se neće prikazivati dok ih eksplicitno ne označite kao **Nije spam**.

Otkrivanje spama može se onemogućiti putem stranice Postavke moderiranja komentara.

### Različiti detektori spama

FastComments podržava tri načina otkrivanja spama:

1. Tradicionalni Naïve-Bayes klasifikator koji se neprekidno trenira i dijeli među svim FastComments.com tenantima.
2. Tradicionalni Naïve-Bayes klasifikator koji se neprekidno trenira i **izoliran** je na vaš tenant.
3. Korištenje LLM-a (velikog jezičnog modela).

Svi imaju pristup dijeljenim i izoliranim Naïve-Bayes klasifikatorima.

Opcija LLM može se odabrati na stranici Postavke moderiranja komentara ako koristite Flex naplatu, budući da se naplaćuje na temelju iskorištenih tokena.

### Faktor povjerenja

FastComments prilagođava filtar za spam za korisnika na temelju toga koliko mu se vjeruje na toj stranici.

Na primjer, ako su administratori puno puta prikvačili njihove komentare, vjerojatno je da je riječ o vrlo pouzdanom korisniku. Ili, ako su dugo član stranice i imaju mnogo komentara, njihov faktor povjerenja također može biti visok.

### SSO

Komentari koje objavljuju SSO korisnici mogu se smatrati spamom i bit će provjereni kao takvi. Izuzetak je ako SSO korisnik ima istu adresu e-pošte kao korisnik tenanta koji ima jednu ili više sljedećih dozvola:

- Vlasnik računa
- Super administrator
- Administrator moderiranja komentara

SSO korisnici s tim dozvolama neće imati svoje komentare provjeravane na spam.

### Ponavljane poruke

FastComments će otkriti i spriječiti ponovljene poruke. Također će otkriti ponovljene poruke koje su vrlo slične kako bi pomogao spriječiti spam. Ovo se ne može onemogućiti jer sprječava zloupotrebu naše platforme. Ako imate visok faktor povjerenja, to se uzima u obzir prilikom sprječavanja ponovljenih poruka.