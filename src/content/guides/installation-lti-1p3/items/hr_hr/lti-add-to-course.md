Nakon što je FastComments registriran u vašem LMS-u, instruktori ga dodaju u kolegije na isti način kao i bilo koji drugi LTI vanjski alat.

#### D2L Brightspace

U području sadržaja kolegija:

1. Kliknite **Dodaj postojeće aktivnosti** > **Vanjski alati za učenje**.
2. Odaberite **FastComments** s popisa.
3. Alat se pojavi kao tema u području sadržaja. Otvorite ga jednom da biste inicijalizirali nit komentara za taj resurs.

#### Moodle

U kolegiju:

1. Uključite **Način uređivanja**.
2. U odjeljku gdje želite komentare, kliknite **Dodaj aktivnost ili resurs**.
3. Odaberite **FastComments** iz odabira aktivnosti.
4. Spremite. Studenti vide nit komentara ugrađenu u odjeljak.

#### Blackboard Learn

U kolegiju:

1. Idite u područje sadržaja.
2. Kliknite **Build Content** > **FastComments** (ispod "Learning Tools").
3. Konfigurirajte naziv i pošaljite.

#### Sakai

Održavatelji stranice dodaju alat kroz **Site Info** > **Manage Tools** > pomaknite se do **External Tools** > odaberite **FastComments** > **Nastavi**.

#### Kako se određuje opseg niti

FastComments stvara zasebnu nit komentara za **(instancu LMS-a, kolegij, poveznicu resursa)**. To znači:

- Dva različita kolegija u istom LMS-u dobivaju zasebne niti, čak i ako koriste isto ime alata.
- Isti FastComments alat koji se koristi na dva mjesta unutar istog kolegija stvara dvije niti.
- Dvije različite Brightspace instalacije pod istim FastComments računom dobivaju odvojene niti - ime hosta LMS-a je dio identifikatora niti.

#### SSO

Studenti ne vide zaslon za prijavu. LMS šalje njihov identitet (ime, email, avatar, uloga) FastCommentsu putem LTI pokretanja, i FastComments ih automatski prijavljuje. Njihovi komentari pripisuju se njihovom LMS računu.

Korisnici s LMS ulogama **Instructor** ili **Administrator** automatski su mapirani na FastComments moderator/admin uloge za nit.