Sada dodajmo kod našeg widgeta.

Kopirajte donji kod. Pobrinite se da ste prijavljeni na [fastcomments.com](https://fastcomments.com) 
i ponovno učitajte ovu stranicu ako niste, tako da će se kod unaprijed popuniti vašim podacima računa; inače će se prikazati demo kod.

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

Sada se vratite u alat za izradu web-stranica i kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gornji kod, a ne isječke iz druge dokumentacije, jer je ovaj isječak posebno prilagođen za Zyro.

Sada biste trebali imati nešto slično sljedećem što izgleda prazno. To je očekivano. Pomaknite pokazivač miša preko područja gdje bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Widget koda dodan</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget koda dodan" />
</div>

Sada povucite widget na željenu veličinu; vidjet ćete kako se pojavljuje:

<div class="screenshot white-bg">
    <div class="title">Promijeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promijeni veličinu" />
</div>

...i sada pregledajte i spremite!