Sada ćemo dodati kod našeg widgeta.

Kopirajte kod ispod. Trebalo bi da budete prijavljeni na [fastcomments.com](https://fastcomments.com) 
i osvežite ovu stranicu ako niste, tako da će se kod unapred popuniti informacijama vašeg naloga, u suprotnom će prikazati demo kod.

Sada ćemo kopirati kod:

[inline-code-attrs-start title = 'Zyro Comments kod'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada se vratite u alat za izradu sajta i kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gore navedeni kod i ne isječke koda iz druge dokumentacije, jer je ovaj isječak posebno prilagođen
za Zyro.

Sada bi trebalo da imate nešto slično sledećem, što izgleda prazno. To je očekivano. Pomaknite miša iznad oblasti
gde bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Dodat widget sa kodom</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodat widget sa kodom" />
</div>

Sada prevucite widget da mu odredite željenu veličinu, videćete da će se pojaviti:

<div class="screenshot white-bg">
    <div class="title">Promeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promeni veličinu" />
</div>

...i sada pregledajte i sačuvajte!