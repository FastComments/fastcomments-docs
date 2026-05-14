D2L Brightspace izlaže Dinamičku registraciju putem administracijskog sučelja LTI Advantage. Trebat će vam administratorski pristup.

#### Otvorite zaslon registracije

1. Prijavite se u svoju Brightspace instancu kao administrator.
2. Idite na **Alati administratora** > **Upravljanje proširivošću** > **LTI Advantage**.
3. Kliknite **Registriraj alat**. (The direct URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Zalijepite URL

Vidjet ćete obrazac za registraciju. Ključno polje je **Endpoint za registraciju pokretanja alata** (neke verzije Brightspacea označavaju ga kao "URL za registraciju pokretanja alata").

Zalijepite FastComments URL za registraciju (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">dohvatite ga ovdje</a>) u to polje. Ostala polja ostavite praznima - popunjava ih FastComments tijekom postupka razmjene ključeva registracije.

Kliknite **Registriraj**.

#### Odobrite alat

Brightspace otvara skočni prozor koji komunicira s FastComments, razmjenjuje ključeve i prikazuje ekran s potvrdom. Skočni prozor se zatvara sam kad registracija završi.

Novi alat se pojavljuje na popisu alata u LTI Advantage. Po zadanim postavkama Brightspace označava nove alate kao **onemogućeno** - prebacite prekidač na **omogućeno** kako bi vaši tečajevi mogli koristiti alat.

#### Dodajte deployment

U Brightspaceu, LTI alati trebaju **deployment** prije nego što se mogu koristiti u tečajevima:

1. Otvorite nedavno registrirani FastComments alat.
2. Kliknite **Pregled deploymenta** > **Novi deployment**.
3. Dajte deploymentu ime (npr. "FastComments - All Courses"), odaberite organizacijske jedinice u kojima bi trebao biti dostupan i spremite.

Nakon prvog pokretanja preko ovog deploymenta, FastComments veže `deployment_id` uz svoj zapis konfiguracije - naknadna pokretanja iz drugog deploymenta pod istim klijentom bit će odbijena osim ako se ponovno ne registrirate.