Kada je FastComments registrovan u vašem LMS-u, instruktori ga dodaju u kurseve na isti način na koji dodaju bilo koji drugi LTI eksterni alat.

#### D2L Brightspace

U području sadržaja kursa:

1. Kliknite **Add Existing Activities** > **External Learning Tools**.
2. Izaberite **FastComments** sa liste.
3. Alat se pojavljuje kao tema u području sadržaja. Otvorite ga jednom da inicijalizujete nit komentara za taj resurs.

#### Moodle

U kursu:

1. Uključite **Edit mode**.
2. U odeljku gde želite komentare, kliknite **Add an activity or resource**.
3. Izaberite **FastComments** iz izbornika aktivnosti.
4. Sačuvajte. Studenti vide nit komentara ugrađenu u taj odeljak.

#### Blackboard Learn

U kursu:

1. Idite u Content Area.
2. Kliknite **Build Content** > **FastComments** (pod "Learning Tools").
3. Podesite ime i pošaljite.

#### Sakai

Održavaoci sajta dodaju alat kroz **Site Info** > **Manage Tools** > skrolujte do **External Tools** > izaberite **FastComments** > **Continue**.

#### How Threads Are Scoped

FastComments kreira posebnu nit komentara po **(LMS instance, course, resource link)**. To znači:

- Dva različita kursa u istom LMS-u dobijaju odvojene niti, čak i ako koriste isti naziv alata.
- Isti FastComments alat korišćen na dva mesta unutar jednog kursa kreira dve niti.
- Dve različite Brightspace instalacije pod istim FastComments nalogom dobijaju odvojene niti — hostname LMS-a je deo identifikatora niti.

#### SSO

Studenti ne vide ekran za prijavu. LMS šalje njihov identitet (ime, email, avatar, uloga) FastComments-u putem LTI pokretanja, i FastComments ih automatski prijavljuje. Njihovi komentari su pripisani njihovom LMS nalogu.

Korisnici sa LMS ulogama **Instructor** ili **Administrator** su automatski mapirani na FastComments moderator/admin uloge za tu nit.