Ovaj primjer koristi prilagođeni kod da bi bio kompatibilan sa Wix-om. **Nećete moći koristiti FastComments isječke koda iz drugih tutorijala.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Otvorite HTML dijalog</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Korak 3: Otvorite HTML dijalog" />
</div>

Copy the following HTML snippet and paste it into the dialog, and click Update:

[inline-code-attrs-start title = 'Isečak koda za Wix komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Korak 3: Zalijepite i sačuvajte</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Korak 3: Zalijepite i sačuvajte" />
</div>

Sada biste trebali vidjeti vrlo malu verziju widgeta za komentare:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Rezultat</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Korak 3: Rezultat" />
</div>

Sledeće ćemo pozicionirati i prilagoditi veličinu da odgovara našoj stranici.