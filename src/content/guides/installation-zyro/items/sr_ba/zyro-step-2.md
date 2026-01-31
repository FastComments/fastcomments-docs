---
Sada ćemo dodati kod našeg widgeta.

Kopirajte dolje navedeni kod. Pobrinite se da ste prijavljeni na [fastcomments.com](https://fastcomments.com) i osvježite ovu stranicu ako niste, kako bi se kod unaprijed popunio informacijama o vašem nalogu; u suprotnom će se prikazati demo kod.

Sada kopirajmo kod:

[inline-code-attrs-start title = 'Kod Zyro komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada se vratite u alat za izradu sajta i kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gore navedeni kod, a ne kodove iz ostale dokumentacije, jer je ovaj isječak posebno prilagođen za Zyro.

Sada biste trebali imati nešto slično sljedećem, što će se činiti praznim. To je očekivano. Pređite mišem preko područja gdje bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Widget s kodom dodat</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget s kodom dodat" />
</div>

Sada povucite widget da postane željene veličine; vidjet ćete ga kako se pojavljuje:

<div class="screenshot white-bg">
    <div class="title">Promijenite veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promijenite veličinu" />
</div>

...i sada pregledajte i sačuvajte!

---