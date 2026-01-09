Ovaj primer koristi prilagođeni kod da bi bio kompatibilan sa Wix-om. **Nećete moći da koristite FastComments primere koda iz drugih tutorijala.**

Otvorite formu da dodate naš prilagođeni HTML dijalog klikom na `Enter Code` i izborom `HTML`:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Otvorite HTML dijalog</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Korak 3: Otvorite HTML dijalog" />
</div>

Kopirajte sledeći HTML isječak i nalepite ga u dijalog, pa kliknite Ažuriraj:

[inline-code-attrs-start title = 'Wix primer koda za komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">Korak 3: Nalepite i sačuvajte</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Korak 3: Nalepite i sačuvajte" />
</div>

Sada biste trebali videti vrlo malu verziju widgeta za komentare:

<div class="screenshot white-bg">
    <div class="title">Korak 3: Rezultat</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Korak 3: Rezultat" />
</div>

Zatim ćemo postaviti i promeniti veličinu ovog tako da odgovara našoj stranici.