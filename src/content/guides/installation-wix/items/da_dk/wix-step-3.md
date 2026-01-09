Dette eksempel bruger brugerdefineret kode for at være kompatibelt med Wix. **Du vil ikke kunne bruge FastComments-kodeudsnittene fra andre vejledninger.**

Åbn formularen for at tilføje vores brugerdefinerede HTML-dialog ved at klikke på `Enter Code` og vælge `HTML`:

<div class="screenshot white-bg">
    <div class="title">Trin 3: Åbn HTML-dialog</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Trin 3: Åbn HTML-dialog" />
</div>

Kopiér følgende HTML-udsnit og indsæt det i dialogen, og klik på Opdater:

[inline-code-attrs-start title = 'Kodeudsnit til Wix-kommentarer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Trin 3: Indsæt og gem</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Trin 3: Indsæt og gem" />
</div>

Du bør nu se en meget lille version af kommentar-widgeten:

<div class="screenshot white-bg">
    <div class="title">Trin 3: Resultatet</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Trin 3: Resultatet" />
</div>

Herefter vil vi placere og tilpasse størrelsen, så den passer til vores side.

---