Sledeće, podesimo stvari tako da se nit komentara mijenja u zavisnosti od trenutne stranice, omogućavajući korisnicima da raspravljaju o sadržaju koji je trenutno prikazan.

Bez narednih koraka, imaćete samo jednu globalnu nit komentara za cijeli sajt - što nije naročito korisno.

#### Dev Mode

Kliknite opciju `Dev Mode` na vrhu ekrana.

<div class="screenshot white-bg">
    <div class="title">Omogući Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Omogući Dev Mode" />
</div>

#### Set the Element ID

Dodaćemo prilagođeni kod da to postignemo, ali prvo moramo novom embed elementu dodijeliti ID da bismo mogli da ga referenciramo.

Nazovimo ga `fastcomments`.

Kliknite novi embed element koji smo dodali, i u dev modu u donjem desnom uglu trebali biste vidjeti polje za ID sa vrijednošću poput `html1`:

<div class="screenshot white-bg">
    <div class="title">Polje ID-a</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Polje ID-a" />
</div>

Promijenite ovo u `fastcomments` i pritisnite Enter:

<div class="screenshot white-bg">
    <div class="title">Postavite ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Postavite ID" />
</div>

Sada možemo dodati naš prilagođeni kod koji govori području za komentare koju stranicu gledamo.

Na dnu ekrana trebali biste vidjeti uređivač koda kao ovaj:

<div class="screenshot white-bg">
    <div class="title">Otvorite uređivač</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Postavite ID" />
</div>

Kopirajte sljedeći kod i zalijepite ga tamo:

[inline-code-attrs-start title = 'Isječak navigacije komentara za Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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