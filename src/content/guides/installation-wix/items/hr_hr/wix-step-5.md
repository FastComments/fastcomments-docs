Zatim ćemo postaviti stvari tako da se nit komentara mijenja ovisno o trenutnoj stranici, čime korisnicima omogućujemo raspravu o trenutno prikazanom sadržaju.

Bez sljedećih koraka, imat ćete samo jednu globalnu nit komentara za cijelu vašu web-lokaciju - što nije baš korisno.

#### Dev Mode

Da bismo dodali ovu funkcionalnost, morat ćemo ući u ono što Wix naziva `Dev Mode`.

Kliknite opciju `Dev Mode` na vrhu zaslona.

<div class="screenshot white-bg">
    <div class="title">Omogući Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Omogući Dev Mode" />
</div>

#### Postavljanje ID-ja elementa

Dodat ćemo prilagođeni kod kako bismo to postigli, ali prvo moramo dati novom ugrađenom elementu ID na koji se možemo pozivati.

Nazovimo ga `fastcomments`.

Kliknite novi ugrađeni element koji smo dodali, i u dev modu u donjem desnom kutu trebali biste vidjeti polje ID s vrijednošću poput `html1`:

<div class="screenshot white-bg">
    <div class="title">Polje ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Polje ID" />
</div>

Promijenite ovo u `fastcomments` i pritisnite enter:

<div class="screenshot white-bg">
    <div class="title">Postavi ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Postavi ID" />
</div>

Sada možemo dodati naš prilagođeni kod koji govori području komentara koju stranicu gledamo.

Na dnu zaslona trebali biste vidjeti uređivač koda poput ovoga:

<div class="screenshot white-bg">
    <div class="title">Otvori uređivač</div>
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
    <div class="title">Dodaj kod za navigaciju</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Dodaj kod za navigaciju" />
</div>