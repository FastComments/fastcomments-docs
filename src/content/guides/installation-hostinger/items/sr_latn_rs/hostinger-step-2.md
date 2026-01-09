Sada ćemo dodati kod našeg widgeta.

Kopirajte kod ispod. Proverite da li ste prijavljeni na [fastcomments.com](https://fastcomments.com) 
i osvežite ovu stranicu ako niste, tako da će kod biti unapred popunjen informacijama vašeg naloga, u suprotnom će prikazati demo kod.

Sada kopirajte kod:

[inline-code-attrs-start title = 'Hostinger kod komentara'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Vratite se u alat za izradu sajta i kliknite na `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gorenavedeni kod i ne isječke iz druge dokumentacije, jer je ovaj isječak posebno
prilagođen za Hostinger.

Sada biste trebali imati nešto slično sledećem, što izgleda prazno. To je očekivano. Pomaknite miš preko oblasti
gde bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Dodat widget</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodat widget" />
</div>

Sada povucite widget da postavite željenu veličinu, videćete da se pojavi:

<div class="screenshot white-bg">
    <div class="title">Promeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promeni veličinu" />
</div>

...i sada pregledajte i sačuvajte!