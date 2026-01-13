Ten przykład używa niestandardowego kodu, aby był zgodny z Wix. **Nie będziesz mógł użyć fragmentów kodu FastComments z innych samouczków.**

Otwórz formularz, aby dodać nasze niestandardowe okno HTML, klikając `Enter Code` i wybierając `HTML`:

<div class="screenshot white-bg">
    <div class="title">Krok 3: Otwórz okno HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Krok 3: Otwórz okno HTML" />
</div>

Skopiuj następujący fragment HTML i wklej go do okna, a następnie kliknij Update:

[inline-code-attrs-start title = 'Fragment kodu komentarzy Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Krok 3: Wklej i Zapisz</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Krok 3: Wklej i Zapisz" />
</div>

Powinieneś teraz zobaczyć bardzo małą wersję widżetu komentarzy:

<div class="screenshot white-bg">
    <div class="title">Krok 3: Wynik</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Krok 3: Wynik" />
</div>

Następnie ustawimy pozycję i rozmiar tak, aby pasowały do naszej strony.