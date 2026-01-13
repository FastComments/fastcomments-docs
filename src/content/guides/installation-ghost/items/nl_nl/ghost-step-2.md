Nu we het zipbestand hebben gedownload, pak het uit naar een map. Ik heb het standaard `casper.zip` gedownload en uitgepakt naar `Downloads\casper` op Windows.

Zorg er vervolgens voor dat je de LTS-versie of een nieuwere versie van NodeJS hebt geïnstalleerd. Je kunt die hier krijgen: https://nodejs.org/en/download/

Zodra NodeJS is geïnstalleerd, moet je een code-editor installeren.

We raden (en gebruiken) Webstorm aan, die je hier kunt krijgen met een proefperiode van 30 dagen (geen creditcard nodig): https://www.jetbrains.com/webstorm/

De volgende beste gratis optie is waarschijnlijk Visual Studio Code: https://code.visualstudio.com/download

Zodra je je editor hebt ingesteld en de thema-map in de editor hebt geopend, open je de terminal in de IDE en voer je het volgende uit:

[inline-code-attrs-start title = 'Installeer het thema'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Een succesvolle uitvoer ziet er als volgt uit (je kunt waarschuwingen negeren):

<div class="screenshot white-bg">
    <div class="title">Succesvolle uitvoer van npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Succesvolle uitvoer van npm install" />
</div>

Dit stelt de afhankelijkheden van het thema in voor de latere opdrachten die we zullen uitvoeren. Bovendien is de export afhankelijk van het feit dat de afhankelijkheden van het thema zijn geïnstalleerd; anders zal de herimport niet correct werken.

---