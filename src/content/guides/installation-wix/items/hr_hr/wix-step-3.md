Ovaj primjer koristi prilagođeni kod kako bi bio kompatibilan s Wixom. **Nećete moći koristiti FastComments isječke koda iz drugih vodiča.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Otvorite HTML dijalog</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Korak 3: Otvorite HTML dijalog" />
</div>

Kopirajte sljedeći HTML isječak i zalijepite ga u dijalog, zatim kliknite Ažuriraj:

[inline-code-attrs-start title = 'Umetak koda za Wix komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Korak 3: Zalijepite i spremite</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Korak 3: Zalijepite i spremite" />
</div>

Sada biste trebali vidjeti vrlo malu verziju widgeta za komentare:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Rezultat</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Korak 3: Rezultat" />
</div>

Sljedeće ćemo to pozicionirati i prilagoditi veličinu tako da odgovara našoj stranici.

---