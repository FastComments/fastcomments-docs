Sada ćemo dodati kod našeg widgeta.

Kopirajte kod ispod. Trebat ćete osigurati da ste prijavljeni na [fastcomments.com](https://fastcomments.com) i osvježiti ovu stranicu ako niste, kako bi se kod unaprijed popunio vašim računom, inače će se prikazati demo kod.

Sada kopirajmo kod:

[inline-code-attrs-start title = 'Hostinger kod komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada se vratimo u naš alat za izradu sajta i kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gornji kod, a ne kod iz drugih uputa, jer je ovaj isječak posebno pripremljen za Hostinger.

Sada biste trebali imati nešto slično sljedećem, što se pojavljuje prazno. To je očekivano. Pomaknite miš iznad područja gdje bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Dodan widget s kodom</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodan widget s kodom" />
</div>

Sada povucite widget na željenu veličinu, vidjet ćete da se pojavi:

<div class="screenshot white-bg">
    <div class="title">Promijeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promijeni veličinu" />
</div>

...a sada pregledajte i sačuvajte!