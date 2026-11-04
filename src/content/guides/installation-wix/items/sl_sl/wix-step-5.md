Nato nastavimo stvari tako, da se nit komentarjev spreminja glede na trenutno stran, kar uporabnikom omogoča razpravo o trenutno prikazani vsebini.

Brez naslednjih korakov boste imeli samo eno globalno nit komentarjev za celotno spletno stran - kar ni zelo uporabno.

#### Dev Mode

Za dodajanje te funkcionalnosti bomo morali vstopiti v tisto, čemur Wix pravi `Dev Mode`.

Kliknite možnost `Dev Mode` na vrhu zaslona.

<div class="screenshot white-bg">
    <div class="title">Omogoči Dev Mode</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Omogoči Dev Mode" />
</div>

#### Nastavite ID elementa

Za dosego tega bomo dodali kodo po meri, vendar moramo najprej novemu vgrajenemu elementu dati ID, na katerega se bomo sklicevali.

Poimenujmo ga `fastcomments`.

Kliknite nov vgrajeni element, ki smo ga dodali, in v dev načinu spodaj desno bi morali videti polje ID z vrednostjo kot `html1`:

<div class="screenshot white-bg">
    <div class="title">Polje ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Polje ID" />
</div>

Spremenite to v `fastcomments` in pritisnite enter:

<div class="screenshot white-bg">
    <div class="title">Nastavitev ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Nastavitev ID" />
</div>

Zdaj lahko dodamo našo kodo po meri, ki povem območju komentarjev, katero stran si ogledujemo.

Na dnu zaslona bi morali videti urejevalnik kode, kot je ta:

<div class="screenshot white-bg">
    <div class="title">Odpiranje urejevalnika</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Nastavitev ID" />
</div>

Kopirajte naslednjo kodo in jo prilepite tja:

[inline-code-attrs-start title = 'Wix odsek za navigacijo komentarjev'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Dodajanje kode za navigacijo</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Dodajanje kode za navigacijo" />
</div>