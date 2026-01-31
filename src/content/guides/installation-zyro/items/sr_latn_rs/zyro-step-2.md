Sada ćemo dodati kod našeg widgeta.

Kopirajte kod ispod. Potrebno je da budete prijavljeni na [fastcomments.com](https://fastcomments.com) i da osvežite ovu stranicu ako niste, kako bi se kod unapred popunio vašim podacima naloga, u suprotnom će prikazati demo kod.

Sada kopirajmo kod:

[inline-code-attrs-start title = 'Zyro Comments kod'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada se vratite u alat za izradu sajta i kliknite na `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gornji kod, a ne isečke koda iz druge dokumentacije, jer je ovaj isječak posebno prilagođen za Zyro.

Sada biste trebali imati nešto slično sledećem, što izgleda prazno. To je očekivano. Pomaknite miš preko područja gde bi trebalo da bude widget:

<div class="screenshot white-bg">
    <div class="title">Widget dodat</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget dodat" />
</div>

Sada prevucite widget dok ne postignete željenu veličinu, videćete da se pojavljuje:

<div class="screenshot white-bg">
    <div class="title">Promeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promeni veličinu" />
</div>

...i sada pregledajte i sačuvajte!