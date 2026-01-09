Ta primer uporablja lastno kodo za združljivost z Wixom. **Ne boste mogli uporabiti odlomkov kode FastComments iz drugih navodil.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Odprite HTML pogovorno okno</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Korak 3: Odprite HTML pogovorno okno" />
</div>

Copy the following HTML snippet and paste it into the dialog, and click Posodobi:

[inline-code-attrs-start title = 'Vstavek kode komentarjev za Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Korak 3: Prilepi in shrani</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Korak 3: Prilepi in shrani" />
</div>

Zdaj bi morali videti zelo majhno različico pripomočka za komentarje:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Rezultat</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Korak 3: Rezultat" />
</div>

Naslednje bomo to postavili in spremenili velikost, da bo ustrezalo naši strani.

---