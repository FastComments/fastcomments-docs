**Koristite Moodle?** Takođe objavljujemo poseban Moodle plugin za FastComments sa bližom integracijom nego LTI 1.3 (grade sync hooks, dublje izveštavanje aktivnosti, nativni Moodle interfejs za podešavanja). Pogledajte <a href="/guide-installation-moodle.html" target="_blank">Vodič za instalaciju Moodle plugina</a>. LTI 1.3 flow ispod je pravi izbor ako želite jedinstvenu registraciju koja pokriva i druge LMS-ove, ili ako vaš Moodle administrator neće instalirati dodatke trećih strana.

Moodle 4.0+ podržava LTI 1.3 Dynamic Registration kroz dodatak External Tool.

#### Otvorite ekran za upravljanje alatima

1. Prijavite se u Moodle kao administrator sajta.
2. Idite na **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Nalepite URL

Videćete karticu označenu **Tool URL**. Nalepite URL za registraciju FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovde</a>) u tekstualno polje i kliknite **Add LTI Advantage**.

Moodle otvara ekran za registraciju koji prikazuje identitet alata i dozvole koje zahteva. Pregledajte i kliknite **Activate** (ili **Register**, u zavisnosti od verzije Moodla).

Iskakajući prozor se zatvara kada se registracija završi; novi FastComments alat pojavljuje se u listi **Tools** sa statusom **Active**.

#### Učinite ga dostupnim

Po defaultu Moodle dodaje nove alate na listu "Course tools" ali ih ne prikazuje u izborniku aktivnosti. Da biste omogućili FastComments na nivou kursa:

1. Kliknite ikonu zupčanika na FastComments pločici.
2. Ispod **Tool configuration usage**, izaberite **Show in activity chooser and as a preconfigured tool**.
3. Sačuvajte.

Instruktori sada mogu dodati FastComments u bilo koji kurs putem **Add an activity or resource** > **FastComments**.