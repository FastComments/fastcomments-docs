**Koristite Moodle?** Objavljujemo i poseban Moodle dodatak za FastComments s dubljom integracijom nego LTI 1.3 (hookovi za sinkronizaciju ocjena, detaljnije izvještavanje aktivnosti, izvorno sučelje za postavke u Moodlu). Pogledajte <a href="/guide-installation-moodle.html" target="_blank">upute za instalaciju Moodle dodatka</a>. LTI 1.3 tijek u nastavku dobar je izbor ako želite jedinstvenu registraciju koja pokriva i ostale LMS-ove, ili ako vaš Moodle administrator neće instalirati dodatke trećih strana.

Moodle 4.0+ podržava LTI 1.3 dinamičku registraciju putem dodatka External Tool.

#### Otvorite zaslon za upravljanje alatima

1. Prijavite se u Moodle kao administrator stranice.
2. Idite na **Administracija stranice** > **Dodaci** > **Moduli aktivnosti** > **Vanjski alat** > **Upravljanje alatima**.

#### Zalijepite URL

Vidjet ćete karticu označenu kao **Tool URL**. Zalijepite FastComments registracijski URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">dohvatite ga ovdje</a>) u tekstualno polje i kliknite **Dodaj LTI Advantage**.

Moodle će otvoriti ekran registracije koji prikazuje identitet alata i dopuštenja koja traži. Pregledajte i kliknite **Aktiviraj** (ili **Registriraj**, ovisno o verziji Moodla).

Skočni prozor se zatvara kada se registracija dovrši; novi FastComments alat pojavljuje se na popisu **Alati** sa statusom **Aktivan**.

#### Učinite ga dostupnim

Po zadanim postavkama Moodle dodaje nove alate na popis **Alati kolegija**, ali ih ne prikazuje u odabiraču aktivnosti. Da biste FastComments učinili dostupnim za cijeli kolegij:

1. Kliknite ikonu zupčanika na FastComments pločici.
2. Pod **Korištenje konfiguracije alata**, odaberite **Prikaži u odabiraču aktivnosti i kao unaprijed konfigurirani alat**.
3. Spremi.

Nastavnici sada mogu dodati FastComments u bilo koji kolegij putem **Dodaj aktivnost ili resurs** > **FastComments**.