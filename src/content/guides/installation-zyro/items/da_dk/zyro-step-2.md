Lad os nu tilføje vores widget-kode.

Kopier koden nedenfor. Du skal sørge for, at du er logget ind på [fastcomments.com](https://fastcomments.com) og genindlæse denne side, hvis ikke, så koden bliver forudfyldt med dine kontooplysninger; ellers vises demo-koden.

Kopier nu koden:

[inline-code-attrs-start title = 'Zyro-kommentarer kode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Gå nu tilbage til din sidebygger og klik på `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Indsæt kode</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Indsæt kode" />
</div>

### Bemærk!

Det er vigtigt, at du bruger koden ovenfor og ikke kodestykkerne fra anden dokumentation, da dette uddrag er udarbejdet specifikt til Zyro.

Du bør nu have noget lignende det følgende, som ser tomt ud. Det er forventet. Flyt musen over det område, hvor widgetten skal være:

<div class="screenshot white-bg">
    <div class="title">Kodewidget tilføjet</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Kodewidget tilføjet" />
</div>

Træk nu widgetten til den ønskede størrelse; du vil se den dukke op:

<div class="screenshot white-bg">
    <div class="title">Ændr størrelse</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Ændr størrelse" />
</div>

...og nu forhåndsvis og gem!