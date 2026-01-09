Nu hvor vi har downloadet zip-filen, udpak den til en mappe. Jeg har downloadet standard `casper.zip` og udpakket til `Downloads\casper` på Windows.

Dernæst bør du sikre dig, at du har LTS- eller en nyere version af NodeJS installeret. Du kan hente den her: https://nodejs.org/en/download/

Når NodeJS er installeret, skal du installere en kodeeditor.

Vi anbefaler (og bruger) Webstorm, som du kan få her med en 30-dages prøveperiode (intet kreditkort kræves): https://www.jetbrains.com/webstorm/

Det næstbedste gratis alternativ er sandsynligvis Visual Studio Code: https://code.visualstudio.com/download

Når du har sat din editor op og har temamappen åben i editoren, åbn terminalen i IDE'en og kør:

[inline-code-attrs-start title = 'Installer temaet'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Succesfuldt output vil se sådan ud (du kan ignorere advarsler):

<div class="screenshot white-bg">
    <div class="title">Succesfuldt npm install-output</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Succesfuldt output fra npm install" />
</div>

Dette vil sætte temaets afhængigheder op til de kommandoer, vi kører senere. Eksporten afhænger også af, at temaets afhængigheder er installeret; ellers vil re-importen ikke fungere korrekt.

---