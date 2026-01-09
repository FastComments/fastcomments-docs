Næste skal vi sætte tingene op, så kommentartråden ændrer sig baseret på den aktuelle side, hvilket tillader brugerne at diskutere det aktuelt viste indhold.

Uden de følgende trin vil du kun have én global kommentartråd for hele dit site - hvilket ikke er særligt nyttigt.

#### Dev Mode

For at tilføje denne funktionalitet må vi gå ind i det, Wix kalder `Dev Mode`.

Klik på indstillingen `Dev Mode` øverst på skærmen.

<div class="screenshot white-bg">
    <div class="title">Aktivér Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Aktivér Dev Mode" />
</div>

#### Angiv elementets ID

Vi vil tilføje tilpasset kode for at gøre dette, men først skal vi give det nye embed-element et ID, som vi kan henvise til.

Lad os kalde det `fastcomments`.

Klik på det nye embed-element, vi tilføjede, og i Dev Mode nederst til højre bør du se et ID-felt med en værdi som `html1`:

<div class="screenshot white-bg">
    <div class="title">ID-feltet</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID-feltet" />
</div>

Ændr dette til `fastcomments` og tryk Enter:

<div class="screenshot white-bg">
    <div class="title">Angiv ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Angiv ID" />
</div>

Nu kan vi tilføje vores tilpassede kode, der fortæller kommentarfeltet, hvilken side vi ser.

Nederst på skærmen skulle du se en kodeeditor som denne:

<div class="screenshot white-bg">
    <div class="title">Åbn editoren</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Åbn editoren" />
</div>

Kopier følgende kode og indsæt den dér:

[inline-code-attrs-start title = 'Wix-kommentarers navigationsudsnit'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Tilføj navigationskoden</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Tilføj navigationskoden" />
</div>

---