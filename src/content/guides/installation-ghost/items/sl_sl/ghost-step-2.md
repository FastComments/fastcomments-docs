Zdaj ko smo prenesli zip datoteko, jo razširite v mapo. Jaz sem prenesel privzeto `casper.zip` in jo razpakiral v `Downloads\casper` na Windows.

Nato se prepričajte, da imate nameščeno LTS ali novejšo različico NodeJS. Prenesete jo tukaj: https://nodejs.org/en/download/

Ko je NodeJS nameščen, boste želeli namestiti urejevalnik kode.

Priporočamo (in uporabljamo) Webstorm, ki ga lahko dobite tukaj z 30-dnevno preizkusno različico (ni potrebna kreditna kartica): https://www.jetbrains.com/webstorm/

Naslednja najboljša brezplačna možnost je verjetno Visual Studio Code: https://code.visualstudio.com/download

Ko imate urejevalnik nastavljen in odprto mapo teme v urejevalniku, odprite terminal v IDE in zaženite:

[inline-code-attrs-start title = 'Namestite temo'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Uspešen izhod bo izgledal tako (opozorila lahko prezrete):

<div class="screenshot white-bg">
    <div class="title">Uspešen izpis npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Uspešen izpis npm install" />
</div>

To bo nastavilo odvisnosti teme za kasnejše ukaze, ki jih bomo izvedli. Prav tako izvoz temelji na tem, da so odvisnosti teme nameščene, sicer ponovni uvoz ne bo pravilno deloval.

---