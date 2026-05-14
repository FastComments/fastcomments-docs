#### Navigirajte do LTI 1.3 konfiguracije

Prijavite se u FastComments i idite na <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">vašu stranicu za LTI 1.3 konfiguraciju</a>.

Ako vaš nalog još nema LTI pristup, videćete "LTI not enabled for this account" - kontaktirajte podršku da bi ga omogućili za vaš plan.

#### Odaberite platformu (opciono)

U okviru **Generate a Dynamic Registration URL**, koristite padajući meni **Platform** da navedete kojem LMS-u se povezujete:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Other LTI 1.3 platform

Takođe možete ostaviti na **Auto-detect**. Platforma se čita iz openid-configuration vašeg LMS-a tokom registracije; padajući meni samo postavlja prikazni naziv za konačnu konfiguraciju.

#### Generišite URL

Kliknite **Generate URL**. FastComments kreira jednokratni registracioni token i prikaže URL koji izgleda ovako:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopirajte ga. Ovaj URL:

- Je **single-use** - jednom kada ga vaš LMS uspješno pozove, token se potroši.
- Ističe nakon **30 minutes** ako nije iskorišćen.
- Treba ga držati privatnim - svako ko ima URL može registrovati alat protiv vašeg tenant-a u tih 30 minuta.

#### Existing Configurations

Kada se registracija uspješno završi, nova konfiguracija će se pojaviti u tabeli **Existing Configurations** na istoj stranici, sa svojim Platform, Issuer, Client ID i Status. Možete obrisati konfiguracije iz ove tabele ako ikada budete trebali odregistrirati.