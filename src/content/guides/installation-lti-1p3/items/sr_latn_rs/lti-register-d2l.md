D2L Brightspace izlaže dinamičku registraciju kroz LTI Advantage administratorski interfejs. Trebaće vam administratorski pristup.

#### Otvorite ekran za registraciju

1. Prijavite se u vašu Brightspace instancu kao administrator.
2. Idite na **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknite **Register Tool**. (Direktan URL je `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Zalepite URL

Videćete obrazac za registraciju. Ključno polje je **Tool initiation registration endpoint** (neke verzije Brightspace-a ga nazivaju "Tool Initiation Registration URL").

Zalepite FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">nabavite ga ovde</a>) u to polje. Ostavite ostala polja prazna — FastComments će ih automatski popuniti tokom postupka razmene pri registraciji.

Kliknite **Register**.

#### Odobrite alat

Brightspace otvara skočni prozor koji komunicira sa FastComments, razmenjuje ključeve i prikazuje ekran za potvrdu. Skočni prozor se sam zatvara kada se registracija završi.

Novi alat će se pojaviti na listi LTI Advantage alata. Podrazumevano Brightspace označava nove alate kao **disabled** — prebacite prekidač na **enabled** da bi vaši kursevi mogli da ga koriste.

#### Dodajte deployment

U Brightspace-u, LTI alati zahtevaju **deployment** pre nego što mogu biti korišćeni u kursevima:

1. Otvorite novo registrovani FastComments tool.
2. Kliknite **View Deployments** > **New Deployment**.
3. Dajte deployment-u ime (npr. "FastComments - All Courses"), izaberite organizacione jedinice u kojima treba da bude dostupan, i sačuvajte.

Nakon prvog pokretanja preko ovog deployment-a, FastComments prikači `deployment_id` na svoj zapis konfiguracije — naredna pokretanja iz drugog deployment-a u okviru istog klijenta biće odbijena osim ako se ponovo ne registrujete.