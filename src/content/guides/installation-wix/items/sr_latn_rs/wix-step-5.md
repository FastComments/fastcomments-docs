Zatim ćemo podesiti stvari tako da se nit komentara menja u zavisnosti od trenutne stranice, čime korisnicima omogućavamo da diskutuju o trenutno prikazanom sadržaju.

Bez sledećih koraka, imaćete samo jedan globalni nit komentara za ceo sajt - što nije baš korisno.

#### Dev Mode

Da bismo dodali ovu funkcionalnost, moraćemo da uđemo u ono što Wix naziva `Dev Mode`.

Kliknite na opciju `Dev Mode` na vrhu ekrana.

<div class="screenshot white-bg">
    <div class="title">Omogućite Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Omogućite Dev Mode" />
</div>

#### Postavite ID elementa

Dodaćemo prilagođeni kod da bismo ovo postigli, ali prvo moramo da damo novom ugnježdenom elementu ID na koji ćemo se pozivati.

Nazvaćemo ga `fastcomments`.

Kliknite na novi ugnježdeni element koji smo dodali, i u dev režimu u donjem desnom uglu trebalo bi da vidite polje ID sa vrednošću poput `html1`:

<div class="screenshot white-bg">
    <div class="title">Polje ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Polje ID" />
</div>

Promenite ovo u `fastcomments` i pritisnite enter:

<div class="screenshot white-bg">
    <div class="title">Postavite ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Postavite ID" />
</div>

Sada možemo da dodamo naš prilagođeni kod koji govori oblasti komentara koju stranicu gledamo.

Na dnu ekrana trebalo bi da vidite editor koda poput ovog:

<div class="screenshot white-bg">
    <div class="title">Otvorite Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Postavite ID" />
</div>

Kopirajte sledeći kod i nalepite ga tamo:

[inline-code-attrs-start title = 'Wix Snippet za Navigaciju Komentara'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Dodajte Kod za Navigaciju</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Dodajte Kod za Navigaciju" />
</div>