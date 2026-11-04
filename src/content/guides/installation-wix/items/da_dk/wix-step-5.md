Dernæst lad os indstille det, så kommentartråden ændrer sig baseret på den aktuelle side, så brugerne kan diskutere det indhold, der vises i øjeblikket.

Uden følgende trin vil du kun have én global kommentartråd for hele dit websted - hvilket ikke er særlig nyttigt.

#### Dev Mode

For at tilføje denne funktionalitet skal vi gå ind i det, Wix kalder `Dev Mode`.

Klik på `Dev Mode`-indstillingen øverst på skærmen.

<div class="screenshot white-bg">
    <div class="title">Aktivér Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Aktivér Dev Mode" />
</div>

#### Sæt Element-ID'et

Vi kommer til at tilføje brugerdefineret kode for at opnå dette, men først skal vi give det nye indlejringselement et ID, som vi kan referere til det med.

Lad os kalde det `fastcomments`.

Klik på det nye indlejringselement, vi tilføjede, og i dev mode nederst til højre bør du se et ID-felt med en værdi som `html1`:

<div class="screenshot white-bg">
    <div class="title">ID-feltet</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID-feltet" />
</div>

Ændr dette til `fastcomments` og tryk enter:

<div class="screenshot white-bg">
    <div class="title">Sæt ID'et</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Sæt ID'et" />
</div>

Nu kan vi tilføje vores brugerdefinerede kode, der fortæller kommentarområdet, hvilken side vi ser.

Nederst på skærmen bør du se en kodeeditor som denne:

<div class="screenshot white-bg">
    <div class="title">Åbn Editoren</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Sæt ID'et" />
</div>

Kopiér følgende kode og indsæt den der:

[inline-code-attrs-start title = 'Wix Kommentarer Navigationssnippet'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Tilføj Navigationskoden</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Tilføj Navigationskoden" />
</div>