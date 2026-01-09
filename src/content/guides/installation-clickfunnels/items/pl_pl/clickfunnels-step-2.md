Teraz, gdy jesteśmy w edytorze szablonów, musimy zdecydować, gdzie chcemy wyświetlać komentarze lub czat na żywo.

W tym przykładzie dodamy go bezpośrednio pod wideo. Najedź kursorem na element, aby dodać widżet na jego końcu, a następnie kliknij `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Dodaj element</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Dodaj element" />
</div>

Wybierz `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Wybierz CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Wybierz CUSTOM JS/HTML" />
</div>

Otwórzmy teraz edytor kodu, w którym wkleimy nasz kod.

W następnym kroku ClickFunnels może być nieco mylący.

Ważne, aby *NIE* wybierać `Code` po najechaniu kursorem na nowy element. Zamiast tego wybierz `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Wybierz SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Wybierz SETTINGS" />
</div>

Po prawej stronie możemy kliknąć `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Kliknij Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Kliknij Open Code Editor" />
</div>

Pojawi się duże okno. To tutaj wkleimy nasz kod. Skopiuj następujący fragment (użyj przycisku kopiowania w prawym górnym rogu):

[inline-code-attrs-start title = 'Fragment kodu czatu strumieniowego ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // niektórzy dostawcy zmieniają fragment kodu, czyniąc go asynchronicznym
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Ten fragment kodu jest przeznaczony dla naszego produktu Streaming Chat, który dobrze współgra z materiałami wideo. Jeśli zamiast tego chcesz fragment kodu widżetu Live Commenting, który najlepiej sprawdza się na zwykłych stronach lub wpisach na blogu, znajduje się on na końcu tego samouczka.

Gdy wkleimy fragment kodu do okna, powinno to wyglądać tak:

<div class="screenshot white-bg">
    <div class="title">Wklej kod</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Wklej kod" />
</div>

Teraz musimy tylko zamknąć okno:

<div class="screenshot white-bg">
    <div class="title">Zamknij</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Zamknij" />
</div>

Teraz możesz podejrzeć swoje zmiany! Przesuwaj widżet i sprawdź, gdzie najbardziej Ci odpowiada.

<div class="screenshot white-bg">
    <div class="title">Podgląd</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Podgląd" />
</div>

Sukces! Nie zapomnij przetestować wersji mobilnej!

<div class="screenshot white-bg">
    <div class="title">Sukces!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Sukces!" />
</div>