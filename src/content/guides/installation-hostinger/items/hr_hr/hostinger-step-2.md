Sada dodajmo kod našeg widgeta.

Kopirajte donji kod. Pobrinite se da ste prijavljeni na [fastcomments.com](https://fastcomments.com) 
i osvježite ovu stranicu ako niste, kako bi se kod unaprijed popunio vašim informacijama o računu, u suprotnom će se prikazati demo kod.

Sada ćemo kopirati kod:

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

Sada se vratite u alat za izradu stranica i kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gornji kod, a ne isječke koda iz druge dokumentacije, jer je ovaj isječak posebno pripremljen za Hostinger.

Sada biste trebali imati nešto slično sljedećem, što se čini praznim. To je očekivano. Pomaknite miš iznad područja gdje bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Widget koda dodan</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget koda dodan" />
</div>

Sada povucite widget da postavite željenu veličinu, vidjet ćete ga kako se pojavljuje:

<div class="screenshot white-bg">
    <div class="title">Promijeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promijeni veličinu" />
</div>

...i sada pregledajte i spremite!