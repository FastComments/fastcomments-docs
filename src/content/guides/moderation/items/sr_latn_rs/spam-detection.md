Po defaultu, FastComments dolazi sa mogućnošću treniranja detekcije neželjene pošte.

Kako moderirate komentare i označavate ih kao **Neželjena pošta**, ili označavate komentare koje sistem automatski pronađe kao **Neželjena pošta** kao **Nije neželjena pošta**, sistem za detekciju neželjene pošte će učiti iz ovih akcija kako bi preciznije odredio šta želite da bude neželjena pošta.

Komentari označeni kao **Neželjena pošta** neće biti automatski odobreni, tako da se neće prikazivati dok ih izričito ne označite kao **Nije neželjena pošta**.

Detekcija neželjene pošte može se onemogućiti preko stranice Podešavanja moderacije komentara.

### Različiti detektori neželjene pošte

FastComments podržava tri načina otkrivanja neželjene pošte:

1. Tradicionalni Naivni-Bayes klasifikator koji se kontinuirano trenira, a koji je deljen između svih FastComments.com tenant-a.
2. Tradicionalni Naivni-Bayes klasifikator koji se kontinuirano trenira, koji je **izolovan** na vaš tenant.
3. Korišćenje ChatGPT 4.

Svi imaju pristup deljenim i izolovanim Naivni-Bayes klasifikatorima.

Opcija ChatGPT 4 je izabriva na stranici Podešavanja moderacije komentara ako ste na Flex naplati, pošto se naplaćuje na osnovu iskorišćenih tokena.

### Faktor poverenja

FastComments prilagođava filter neželjene pošte za korisnika na osnovu toga koliko je korisnik pouzdan za dati sajt.

Na primer, ako su administratori prikačili mnogo njihovih komentara, verovatno je da je u pitanju veoma pouzdan korisnik. Ili, ako je korisnik dugo član sajta i ima mnogo komentara, njegov faktor poverenja takođe može biti visok.

### SSO

Komentari koje postave SSO korisnici mogu se smatrati neželjenom poštom i biće proveravani kao takvi. Izuzetak je ako SSO korisnik ima isti email kao korisnik tenanta koji ima jednu ili više od sledećih dozvola:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO korisnici sa ovim dozvolama neće imati svoje komentare proveravane na neželjenu poštu.

### Ponavljanje poruka

FastComments će detektovati i sprečiti ponavljane poruke. Takođe će detektovati ponavljene poruke koje su veoma slične kako bi pomogao u sprečavanju neželjene pošte. Ovo se ne može onemogućiti jer sprečava zloupotrebu naše platforme. Ako imate visok faktor poverenja, to se uzima u obzir prilikom sprečavanja ponavljanja poruka.