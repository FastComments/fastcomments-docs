---
Sledeće, hajde da podesimo stvari tako da se nit komentara menja u zavisnosti od trenutne stranice, omogućavajući korisnicima da diskutuju o sadržaju koji je trenutno prikazan.

Bez sledećih koraka imaćete samo jednu globalnu nit komentara za ceo sajt — što nije naročito korisno.

#### Dev Mode

Da bismo dodali ovu funkcionalnost, moraćemo da uđemo u ono što Wix naziva `Dev Mode`.

Kliknite opciju `Dev Mode` na vrhu ekrana.

<div class="screenshot white-bg">
    <div class="title">Omogući Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Omogući Dev Mode" />
</div>

#### Podesite ID elementa

Dodaćemo prilagođeni kod da bismo to postigli, ali prvo moramo novom embed elementu dodeliti ID po kojem ćemo se na njega pozivati.

Nazovimo ga `fastcomments`.

Kliknite novi embed element koji smo dodali, i u `Dev Mode` u donjem desnom uglu trebalo bi da vidite polje za ID sa vrednošću poput `html1`:

<div class="screenshot white-bg">
    <div class="title">Polje za ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Polje za ID" />
</div>

Promenite ovo u `fastcomments` i pritisnite enter:

<div class="screenshot white-bg">
    <div class="title">Postavite ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Postavite ID" />
</div>

Sada možemo dodati naš prilagođeni kod koji polju komentara govori koju stranicu trenutno prikazujemo.

Na dnu ekrana trebalo bi da vidite editor koda poput ovog:

<div class="screenshot white-bg">
    <div class="title">Otvorite editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Postavite ID" />
</div>

Kopirajte sledeći kod i nalepite ga tamo:

[inline-code-attrs-start title = 'Wix navigacioni isječak za komentare'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Dodajte navigacioni kod</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Dodajte navigacioni kod" />
</div>

---