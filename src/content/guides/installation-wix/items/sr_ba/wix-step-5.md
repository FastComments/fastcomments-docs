Next, hajde da podesimo stvari tako da se nit komentara mijenja u zavisnosti od trenutne stranice, omogućavajući korisnicima da diskutuju o trenutno prikazanom sadržaju.

Bez sljedećih koraka, imaćete samo jednu globalnu nit komentara za cijeli sajt - što nije baš korisno.

#### Dev režim

Da bismo dodali ovu funkcionalnost, morat ćemo ući u ono što Wix zove `Dev Mode`.

Kliknite opciju `Dev Mode` na vrhu ekrana.

<div class="screenshot white-bg">
    <div class="title">Omogući Dev režim</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Omogući Dev režim" />
</div>

#### Postavi ID elementa

Dodaćemo prilagođeni kod da to postignemo, ali prvo moramo novom ugrađenom elementu dati ID po kojem ćemo ga referencirati.

Nazovimo ga `fastcomments`.

Kliknite novi ugrađeni element koji smo dodali, i u dev modu u donjem desnom uglu trebali biste vidjeti polje za ID sa vrijednošću poput `html1`:

<div class="screenshot white-bg">
    <div class="title">Polje ID-a</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Polje ID-a" />
</div>

Promijenite ovo u `fastcomments` i pritisnite enter:

<div class="screenshot white-bg">
    <div class="title">Postavi ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Postavi ID" />
</div>

Sada možemo dodati naš prilagođeni kod koji govori području komentara koju stranicu trenutno gledamo.

Na dnu ekrana trebali biste vidjeti uređivač koda kao ovaj:

<div class="screenshot white-bg">
    <div class="title">Otvorite editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Postavi ID" />
</div>

Kopirajte sljedeći kod i zalijepite ga tamo:

[inline-code-attrs-start title = 'Wix isječak za navigaciju komentara'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Dodajte kod za navigaciju</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Dodajte kod za navigaciju" />
</div>

---