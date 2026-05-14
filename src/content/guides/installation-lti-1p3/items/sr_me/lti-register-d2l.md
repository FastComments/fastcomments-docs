D2L Brightspace izlaže Dynamic Registration preko LTI Advantage administratorskog interfejsa. Trebaće vam administratorski pristup.

#### Open the Registration Screen

1. Prijavite se u svoju Brightspace instancu kao admin.
2. Navigirajte do **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknite **Register Tool**. (Direktan URL je `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Videćete formular za registraciju. Ključno polje je **Tool initiation registration endpoint** (neke verzije Brightspace-a ga nazivaju "Tool Initiation Registration URL").

Zalijepite FastComments registration URL u to polje. Ostavite ostala polja prazna - FastComments ih popunjava automatski tokom handshake procesa registracije.

Kliknite **Register**.

#### Approve the Tool

Brightspace otvara popup koji komunicira sa FastComments, razmjenjuje ključeve i prikazuje ekran za potvrdu. Popup se zatvara sam kada je registracija završena.

Novi alat će se pojaviti na listi vaših LTI Advantage alata. Po defaultu Brightspace označava nove alate kao **disabled** - prebacite prekidač na **enabled** da bi vaši kursevi mogli da ga koriste.

#### Add a Deployment

U Brightspace-u, LTI alati trebaju imati **deployment** prije nego što mogu biti korišćeni u kursevima:

1. Otvorite novoregistrovani FastComments alat.
2. Kliknite **View Deployments** > **New Deployment**.
3. Dajte deploymentu ime (npr. "FastComments - All Courses"), izaberite org jedinice u kojima treba da bude dostupan i sačuvajte.

Nakon prvog pokretanja kroz ovaj deployment, FastComments pričvršćuje `deployment_id` za svoj konfigurisani zapis - naredna pokretanja iz drugog deploymenta pod istim klientom biće odbijena osim ako ponovo ne registrujete.