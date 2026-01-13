Sada kada smo preuzeli zip fajl, raspakujte ga u folder. Ja sam preuzeo podrazumijevani `casper.zip` i raspakovao ga u `Downloads\casper` na Windowsu.

Zatim, uvjerite se da imate instaliranu LTS ili noviju verziju NodeJS-a. Možete je preuzeti ovdje: https://nodejs.org/en/download/

Kada je NodeJS instaliran, treba da instalirate uređivač koda.

Preporučujemo (i koristimo) Webstorm, koji možete dobiti ovdje sa 30-dnevnom probom (nije potrebna kreditna kartica): https://www.jetbrains.com/webstorm/

Sljedeća najbolja besplatna opcija vjerovatno bi bila Visual Studio Code: https://code.visualstudio.com/download

Kada postavite uređivač i otvorite folder teme u uređivaču, otvorite terminal u IDE-u i pokrenite:

[inline-code-attrs-start title = 'Instalirajte temu'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Uspješan izlaz će izgledati ovako (možete ignorisati upozorenja):

<div class="screenshot white-bg">
    <div class="title">Successful npm install output</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Successful npm install output" />
</div>

Ovo će postaviti zavisnosti teme za naredne komande koje ćemo pokretati. Takođe, izvoz zavisi od toga da su zavisnosti teme instalirane, inače ponovni uvoz neće raditi ispravno.

---