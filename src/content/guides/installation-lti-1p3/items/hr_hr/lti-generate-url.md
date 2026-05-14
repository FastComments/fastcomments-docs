#### Navigirajte do konfiguracije LTI 1.3

Prijavite se u FastComments i otvorite <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">svoju stranicu konfiguracije LTI 1.3</a>.

Ako vaš račun još nema LTI pristup, vidjet ćete "LTI not enabled for this account" - kontaktirajte podršku da ga omoguće na vašem planu.

#### Odaberite platformu (neobavezno)

U odjeljku **Generirajte dinamički URL za registraciju**, upotrijebite padajući izbornik **Platforma** da biste FastCommentsu rekli na koji LMS se povezujete:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Druga LTI 1.3 platforma

Također ga možete ostaviti na **Auto-detect**. Platforma se očitava iz openid-configuration vašeg LMS-a tijekom registracije; padajući izbornik samo postavlja prikaznu oznaku za nastalu konfiguraciju.

#### Generirajte URL

Kliknite **Generirajte URL**. FastComments stvara jednokratni registracijski token i prikazuje vam URL koji izgleda ovako:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopirajte ga. Ovaj URL:

- Je namijenjen **jednokratnoj upotrebi** - nakon što ga vaš LMS uspješno pozove, token se potroši.
- Istekne nakon **30 minuta** ako se ne upotrijebi.
- Treba ga držati privatnim - svatko s tim URL-om može registrirati alat za vaš tenant u tih 30 minuta.

#### Postojeće konfiguracije

Kada se registracija uspješno dovrši, nova konfiguracija pojavit će se u tablici **Postojeće konfiguracije** na istoj stranici, sa svojom Platformom, Izdavačem, ID-om klijenta i Statusom. Iz ove tablice možete izbrisati konfiguracije ako ikada budete trebali odjaviti alat.