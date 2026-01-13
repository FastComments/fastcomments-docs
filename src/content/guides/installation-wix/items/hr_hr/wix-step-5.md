Sljedeće, postavimo stvari tako da se nit komentara mijenja ovisno o trenutačnoj stranici, što korisnicima omogućuje raspravu o trenutno prikazanom sadržaju.

Bez sljedećih koraka imat ćete samo jednu globalnu nit komentara za cijelu stranicu - što nije naročito korisno.

#### Dev Mode

Da bismo dodali ovu funkcionalnost, morat ćemo ući u ono što Wix naziva `Dev Mode`.

Kliknite opciju `Dev Mode` na vrhu zaslona.

<div class="screenshot white-bg">
    <div class="title">Omogućite Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Omogućite Dev Mode" />
</div>

#### Set the Element ID

Dodavat ćemo prilagođeni kôd za ostvarenje ovoga, ali prvo moramo novom elementu za ugrađivanje dodijeliti ID kojim ćemo ga referencirati.

Nazovimo ga `fastcomments`.

Kliknite novi element za ugrađivanje koji smo dodali, i u dev modu u donjem desnom kutu trebali biste vidjeti polje ID-a s vrijednošću poput `html1`:

<div class="screenshot white-bg">
    <div class="title">Polje ID-a</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Polje ID-a" />
</div>

Promijenite ovo u `fastcomments` i pritisnite enter:

<div class="screenshot white-bg">
    <div class="title">Postavite ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Postavite ID" />
</div>

Sada možemo dodati naš prilagođeni kôd koji područje komentara obavještava koju stranicu gledamo.

Na dnu zaslona trebali biste vidjeti uređivač kôda poput ovog:

<div class="screenshot white-bg">
    <div class="title">Otvorite uređivač</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Postavite ID" />
</div>

Kopirajte sljedeći kôd i zalijepite ga tamo:

[inline-code-attrs-start title = 'Isječak za navigaciju komentara Wix-a'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Dodajte navigacijski kôd</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Dodajte navigacijski kôd" />
</div>

---