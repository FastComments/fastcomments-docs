#### Idite na LTI 1.3 konfiguraciju

Prijavite se na FastComments i idite na <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">vašu stranicu za LTI 1.3 konfiguraciju</a>.

Ako vaš nalog još nema LTI pristup, videćete "LTI nije omogućen za ovaj nalog" - kontaktirajte podršku da ga omoguće na vašem planu.

#### Izaberite platformu (opciono)

Pod **Generiši dinamički URL za registraciju**, koristite padajući meni **Platforma** da kažete FastComments kojoj LMS platformi se povezujete:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Druga LTI 1.3 platforma

Takođe možete ostaviti na **Automatsko otkrivanje**. Platforma se čita iz vašeg LMS-ovog openid-configuration tokom registracije; padajući meni samo postavlja oznaku koja će se prikazati za nastalu konfiguraciju.

#### Generišite URL

Kliknite **Generiši URL**. FastComments kreira jednokratni token za registraciju i prikaže vam URL koji izgleda ovako:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopirajte ga. Ovaj URL:

- Je **za jednokratnu upotrebu** - nakon što vaš LMS uspešno pozove URL, token se potroši.
- Ističe posle **30 minuta** ako se ne iskoristi.
- Treba ga držati privatnim - svako ko ima URL može registrovati alat za vaš tenant u roku od tih 30 minuta.

#### Postojeće konfiguracije

Kada se registracija uspešno završi, nova konfiguracija se pojavljuje u tabeli **Postojeće konfiguracije** na istoj stranici, sa svojom Platformom, Issuer-om, Client ID-jem i Statusom. Možete izbrisati konfiguracije iz ove tabele ako ikada budete morali da se odregistrujete.