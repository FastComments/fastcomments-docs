Sada ćemo dodati kod našeg widgeta.

Kopirajte kod ispod. Trebalo bi da se ulogujete na [fastcomments.com](https://fastcomments.com) 
i osvježite ovu stranicu ako niste, kako bi se kod unaprijed popunio informacijama vašeg naloga, u suprotnom će prikazati demo kod.

Sada ćemo kopirati kod:

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

Sada se vratimo u alat za izradu sajta i kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesite kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesite kod" />
</div>

### Napomena!

Važno je da koristite gornji kod a ne isječke koda iz druge dokumentacije, jer je ovaj isječak posebno pripremljen
za Hostinger.

Sada biste trebali imati nešto slično sljedećem, što izgleda prazno. To je očekivano. Pomjerite miš preko područja
gdje bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Dodat widget koda</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodat widget koda" />
</div>

Sada povucite widget da postigne željenu veličinu, videćete da će se pojaviti:

<div class="screenshot white-bg">
    <div class="title">Promijeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promijeni veličinu" />
</div>

...i sada pregledajte i sačuvajte!