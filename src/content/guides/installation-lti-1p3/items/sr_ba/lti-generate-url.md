#### Navigate to LTI 1.3 Configuration

Prijavite se na FastComments i idite na <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">vašu LTI 1.3 konfiguracionu stranicu</a>.

Ako vaš račun još nema LTI pristup, videćete "LTI not enabled for this account" - kontaktirajte podršku da ga omoguće na vašem planu.

#### Pick a Platform (Optional)

U okviru **Generate a Dynamic Registration URL**, koristite padajući izbornik **Platform** da kažete FastComments kojem LMS-u se povezujete:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Other LTI 1.3 platform

Možete ga također ostaviti na **Auto-detect**. Platforma se čita iz openid-configuration vašeg LMS-a tokom registracije; padajući izbornik samo postavlja prikazni naziv za rezultirajuću konfiguraciju.

#### Generate the URL

Kliknite **Generate URL**. FastComments kreira jednokratni token za registraciju i prikazuje vam URL koji izgleda ovako:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopirajte ga. Ovaj URL:

- Je **za jednokratnu upotrebu** - nakon što ga vaš LMS uspješno pozove, token se potroši.
- Ističe nakon **30 minuta** ako se ne iskoristi.
- Treba ga držati privatnim - svako ko ima URL može registrovati alat za vaš tenant u tih 30 minuta.

#### Existing Configurations

Kada se registracija uspješno završi, nova konfiguracija će se pojaviti u tabeli **Existing Configurations** na istoj stranici, sa svojim Platform, Issuer, Client ID, i Status. Možete izbrisati konfiguracije iz ove tabele ako ikad budete trebali otkazati registraciju.