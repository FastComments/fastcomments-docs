Sada kad smo preuzeli zip datoteku, raspakirajte je u mapu. Preuzeo sam zadani `casper.zip` i raspakirao ga u `Downloads\casper` na Windowsu.

Sljedeće, trebate provjeriti imate li instaliranu LTS ili noviju verziju NodeJS-a. Možete je preuzeti ovdje: https://nodejs.org/en/download/

Kad je NodeJS instaliran, trebate instalirati uređivač koda.

Preporučujemo (i koristimo) Webstorm, kojeg možete nabaviti ovdje s 30‑dnevnom probom (nije potrebna kreditna kartica): https://www.jetbrains.com/webstorm/

Sljedeća najbolja besplatna opcija vjerojatno je Visual Studio Code: https://code.visualstudio.com/download

Kad imate postavljen uređivač i otvoreni folder teme u uređivaču, otvorite terminal u IDE‑u i pokrenite:

[inline-code-attrs-start title = 'Instalirajte temu'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Uspješan ispis izgledat će ovako (možete ignorirati upozorenja):

<div class="screenshot white-bg">
    <div class="title">Uspješan izlaz naredbe npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Uspješan izlaz naredbe npm install" />
</div>

Ovo će postaviti ovisnosti teme za kasnije naredbe koje ćemo pokrenuti. Također, izvoz ovisi o tome da su ovisnosti teme instalirane; u protivnom ponovni uvoz neće pravilno funkcionirati.

---