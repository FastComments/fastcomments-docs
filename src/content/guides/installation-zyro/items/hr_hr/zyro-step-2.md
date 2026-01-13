---
Sada dodajmo kôd našeg widgeta.

Kopirajte donji kôd. Provjerite jeste li prijavljeni na [fastcomments.com](https://fastcomments.com) 
i osvježite ovu stranicu ako niste, kako bi se kôd unaprijed popunio podacima vašeg računa; u suprotnom prikazat će se demo kôd.

Sada kopirajmo kôd:

[inline-code-attrs-start title = 'Kôd komentara Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada se vratimo u alat za izradu stranice i kliknemo `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesi kôd</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesi kôd" />
</div>

### Napomena!

Važno je da koristite gornji kôd, a ne isječke kôda iz druge dokumentacije, jer je ovaj isječak posebno prilagođen za Zyro.

Sada biste trebali imati nešto slično sljedećem, što se prikazuje prazno. To je očekivano. Pomaknite miš iznad područja gdje bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Widget s kôdom je dodan</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget s kôdom je dodan" />
</div>

Sada povucite widget kako biste mu postavili željenu veličinu — vidjet ćete kako se pojavljuje:

<div class="screenshot white-bg">
    <div class="title">Promijeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promijeni veličinu" />
</div>

...i sada pregledajte i spremite!

---