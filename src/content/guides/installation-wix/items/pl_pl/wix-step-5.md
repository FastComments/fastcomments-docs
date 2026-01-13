Następnie skonfigurujmy to tak, aby wątek komentarzy zmieniał się w zależności od aktualnej strony, pozwalając użytkownikom na dyskusję nad aktualnie wyświetlaną treścią.

Bez poniższych kroków będziesz mieć tylko jeden globalny wątek komentarzy dla całej witryny — co nie jest zbyt przydatne.

#### Tryb deweloperski

Aby dodać tę funkcjonalność, musimy przejść do tego, co Wix nazywa `Dev Mode`.

Kliknij opcję `Dev Mode` u góry ekranu.

<div class="screenshot white-bg">
    <div class="title">Włącz Tryb deweloperski</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Włącz Tryb deweloperski" />
</div>

#### Ustaw ID elementu

Dodamy niestandardowy kod, aby to osiągnąć, ale najpierw musimy nadać nowemu elementowi osadzenia ID, aby można było się do niego odwoływać.

Nazwijmy go `fastcomments`.

Kliknij nowy element osadzenia, który dodaliśmy, a w trybie deweloperskim w prawym dolnym rogu powinieneś zobaczyć pole ID z wartością taką jak `html1`:

<div class="screenshot white-bg">
    <div class="title">Pole ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="Pole ID" />
</div>

Zmień to na `fastcomments` i naciśnij Enter:

<div class="screenshot white-bg">
    <div class="title">Ustaw ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Ustaw ID" />
</div>

Teraz możemy dodać nasz niestandardowy kod, który poinformuje obszar komentarzy, jaką stronę wyświetlamy.

Na dole ekranu powinieneś zobaczyć edytor kodu podobny do tego:

<div class="screenshot white-bg">
    <div class="title">Otwórz edytor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Otwórz edytor" />
</div>

Skopiuj poniższy kod i wklej go tam:

[inline-code-attrs-start title = 'Fragment nawigacji komentarzy Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Dodaj kod nawigacji</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Dodaj kod nawigacji" />
</div>

---