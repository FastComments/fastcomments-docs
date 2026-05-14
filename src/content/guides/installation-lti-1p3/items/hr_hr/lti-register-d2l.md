D2L Brightspace izlaže Dinamičku registraciju putem administratorskog sučelja LTI Advantage. Trebat će vam administratorski pristup.

#### Otvorite zaslon za registraciju

1. Prijavite se u vašu Brightspace instancu kao administrator.
2. Idite na **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknite **Register Tool**. (Izravni URL je `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Zalijepite URL

Vidjet ćete obrazac za registraciju. Ključno polje je **Tool initiation registration endpoint** (neke verzije Brightspacea označavaju ga kao "Tool Initiation Registration URL").

Zalijepite FastComments registracijski URL u to polje. Ostala polja ostavite praznima - FastComments će ih automatski popuniti tijekom ruke za registraciju.

Kliknite **Register**.

#### Odobrite alat

Brightspace otvara skočni prozor koji komunicira s FastComments, razmjenjuje ključeve i prikazuje zaslon za potvrdu. Skočni prozor se zatvara sam kada je registracija dovršena.

Novi alat pojavit će se na popisu vaših LTI Advantage alata. Po zadanom Brightspace označava nove alate kao **onemogućeno** - prebacite prekidač na **omogućeno** kako bi vaši kolegiji mogli koristiti alat.

#### Dodajte implementaciju

U Brightspaceu, LTI alati trebaju **implementaciju** prije nego što se mogu koristiti u kolegijima:

1. Otvorite novo registrirani FastComments alat.
2. Kliknite **View Deployments** > **New Deployment**.
3. Dajte implementaciji ime (npr. "FastComments - All Courses"), odaberite organizacijske jedinice u kojima treba biti dostupan i spremite.

Nakon prvog pokretanja kroz ovu implementaciju, FastComments pričvrsti `deployment_id` u svoj zapis konfiguracije - naknadna pokretanja iz druge implementacije pod istim klijentom bit će odbijena osim ako se ponovno ne registrirate.