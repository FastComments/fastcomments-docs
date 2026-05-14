D2L Brightspace izlaže Dynamic Registration kroz LTI Advantage administratorski interfejs. Biće vam potreban administratorski pristup.

#### Open the Registration Screen

1. Prijavite se u vašu Brightspace instancu kao administrator.
2. Idite na **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknite **Register Tool**. (Direktan URL je `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Videćete formular za registraciju. Ključno polje je **Tool initiation registration endpoint** (neke verzije Brightspace-a ga označavaju kao "Tool Initiation Registration URL").

Zalepite FastComments registration URL u to polje. Ostala polja ostavite prazna — FastComments će ih automatski popuniti tokom registracionog handshaka.

Kliknite **Register**.

#### Approve the Tool

Brightspace otvara popup koji komunicira sa FastComments, razmenjuje ključeve i prikazuje ekran sa potvrdom. Popup se zatvara automatski kada se registracija završi.

Novi alat će se pojaviti na listi vaših LTI Advantage alata. Po defaultu Brightspace označava nove alate kao **disabled** — prebacite prekidač na **enabled** kako bi vaši kursevi mogli da ga koriste.

#### Add a Deployment

U Brightspace-u, LTI alati moraju imati **deployment** pre nego što mogu da se koriste u kursевima:

1. Otvorite novo registrovani FastComments alat.
2. Kliknite **View Deployments** > **New Deployment**.
3. Dajte deployment-u ime (npr. "FastComments - All Courses"), izaberite org jedinice u kojima treba da bude dostupan i sačuvajte.

Nakon prvog pokretanja kroz ovaj deployment, FastComments zakači `deployment_id` u svoj zapis konfiguracije - naknadna pokretanja iz drugog deployment-a pod istim klijentom biće odbijena osim ako ponovo ne registrujete.