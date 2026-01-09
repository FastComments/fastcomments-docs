Sada kada smo preuzeli zip fajl, otpakujte ga u folder. Ja sam preuzeo podrazumevani `casper.zip` i otpakovao u `Downloads\casper` na Windows.

Zatim, treba da se uverite da imate instaliranu LTS ili noviju verziju NodeJS-a. Možete je preuzeti ovde: https://nodejs.org/en/download/

Kada je NodeJS instaliran, treba da instalirate uređivač koda.

Preporučujemo (i mi koristimo) Webstorm, koji možete nabaviti ovde uz probni period od 30 dana (nije potrebna kreditna kartica): https://www.jetbrains.com/webstorm/

Sledeća najbolja besplatna opcija verovatno bi bila Visual Studio Code: https://code.visualstudio.com/download

Kada imate podešen editor i otvoren folder teme u editoru, otvorite terminal u IDE-u i pokrenite:

[inline-code-attrs-start title = 'Instalirajte temu'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Uspešan izlaz će izgledati ovako (možete ignorisati upozorenja):

<div class="screenshot white-bg">
    <div class="title">Uspešan ispis komande npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Uspešan ispis komande npm install" />
</div>

Ovo će podesiti zavisnosti teme za naredne komande koje ćemo pokretati. Takođe, izvoz zavisi od toga da su zavisnosti teme instalirane, u suprotnom ponovni uvoz neće raditi ispravno.