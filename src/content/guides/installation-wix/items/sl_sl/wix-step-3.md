Ta primer uporablja prilagojeno kodo za združljivost z Wix. **Kosov kode FastComments iz drugih vodičev ne boste mogli uporabiti.**

Odprite obrazec za dodajanje našega prilagojenega HTML pogovornega okna s klikom na `Enter Code` in izbiro `HTML`:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Odpri HTML pogovorno okno</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Korak 3: Odpri HTML pogovorno okno" />
</div>

Kopirajte naslednji HTML delček, prilepite ga v pogovorno okno in kliknite Update:

[inline-code-attrs-start title = 'Kos kode za komentarje Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Korak 3: Prilepite in shranite</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Korak 3: Prilepite in shranite" />
</div>

Zdaj bi morali videti zelo majhno različico pripomočka za komentarje:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Rezultat</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Korak 3: Rezultat" />
</div>

Nato bomo to pozicionirali in prilagodili velikost, da bo ustrezala naši strani.