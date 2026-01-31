Sada ćemo dodati kod za naš widget.

Kopirajte kod ispod. Uverite se da ste prijavljeni na [fastcomments.com](https://fastcomments.com) 
i osvežite ovu stranicu ako niste, kako bi kod bio unapred popunjen podacima vašeg naloga, inače će prikazati demo kod.

Sada kopirajmo kod:

[inline-code-attrs-start title = 'Kod komentara za Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada se vratimo u naš uređivač sajta i kliknemo `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesi kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesi kod" />
</div>

### Napomena!

Važno je da koristite gornji kod, a ne fragmente koda iz druge dokumentacije, jer je ovaj isječak posebno prilagođen za Hostinger.

Sada bi trebalo da imate nešto slično sledećem, što izgleda prazno. To je očekivano. Pomaknite miša preko oblasti gde bi widget trebalo da bude:

<div class="screenshot white-bg">
    <div class="title">Widget koda dodat</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget koda dodat" />
</div>

Sada prevucite widget da postignete željenu veličinu — videćete da se pojavio:

<div class="screenshot white-bg">
    <div class="title">Promeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promeni veličinu" />
</div>

...i sada pregledajte i sačuvajte!

---