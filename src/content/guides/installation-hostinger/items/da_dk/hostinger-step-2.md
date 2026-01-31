Lad os nu tilføje vores widget-kode.

Kopier koden nedenfor. Sørg for, at du er logget ind på [fastcomments.com](https://fastcomments.com) og genindlæs denne side, hvis ikke, så koden bliver forudfyldt med dine kontooplysninger, ellers vises demo-koden.

Kopier nu koden:

[inline-code-attrs-start title = 'Hostinger kommentarkode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Gå nu tilbage til din sidebygger og klik på `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Indtast kode</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Indtast kode" />
</div>

### Bemærk!

Det er vigtigt, at du bruger ovenstående kode og ikke kodeudsnit fra anden dokumentation, da dette uddrag er lavet specifikt til Hostinger.

Du skulle nu have noget lignende følgende, som ser tomt ud. Dette er forventet. Før musen over det område, hvor widget'en skal være:

<div class="screenshot white-bg">
    <div class="title">Kode-widget tilføjet</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Kode-widget tilføjet" />
</div>

Træk nu widget'en til den ønskede størrelse, så vil du se den dukke op:

<div class="screenshot white-bg">
    <div class="title">Ændr størrelse</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Ændr størrelse" />
</div>

...og nu forhåndsvis og gem!