Laten we nu alles instellen zodat de commentaardraad verandert op basis van de huidige pagina, zodat gebruikers de momenteel weergegeven inhoud kunnen bespreken.

Zonder de volgende stappen heb je slechts één globale commentaardraad voor je hele site - wat niet erg nuttig is.

#### Dev Mode

Om deze functionaliteit toe te voegen, moeten we naar wat Wix `Dev Mode` noemt.

Klik op de optie `Dev Mode` bovenaan het scherm.

<div class="screenshot white-bg">
    <div class="title">Dev Mode inschakelen</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Dev Mode inschakelen" />
</div>

#### Stel het Element ID in

We gaan aangepaste code toevoegen om dit te bereiken, maar eerst moeten we het nieuwe embed-element een ID geven om ernaar te verwijzen.

Laten we het `fastcomments` noemen.

Klik op het nieuwe embed-element dat we hebben toegevoegd, en in dev mode zie je rechtsonder een ID-veld met een waarde zoals `html1`:

<div class="screenshot white-bg">
    <div class="title">Het ID-veld</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Het ID-veld" />
</div>

Wijzig dit in `fastcomments` en druk op enter:

<div class="screenshot white-bg">
    <div class="title">Stel de ID in</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Stel de ID in" />
</div>

Nu kunnen we onze aangepaste code toevoegen die de commentaarzone vertelt welke pagina we bekijken.

Onderaan het scherm zou je een code-editor als deze moeten zien:

<div class="screenshot white-bg">
    <div class="title">Open de editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Stel de ID in" />
</div>

Kopieer de volgende code en plak deze daar:

[inline-code-attrs-start title = 'Wix Comments Navigation Snippet'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Voeg de navigatiecode toe</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Voeg de navigatiecode toe" />
</div>