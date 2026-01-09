Sada dodajmo kod našeg widgeta.

Kopirajte kod u nastavku. Trebat ćete se pobrinuti da ste prijavljeni na [fastcomments.com](https://fastcomments.com) i ponovno učitajte ovu stranicu ako niste, tako da će kod biti unaprijed popunjen podacima vašeg računa, u protivnom će se prikazivati demo kod.

Sada kopirajmo kod:

[inline-code-attrs-start title = 'Kod komentara za Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada se vratite u svoj site builder i kliknite `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Unesi kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Unesi kod" />
</div>

### Napomena!

Važno je da koristite gore navedeni kod, a ne isječke koda iz druge dokumentacije, jer je ovaj isječak posebno izrađen za Hostinger.

Sada biste trebali imati nešto slično sljedećem, što izgleda prazno. To je očekivano. Pomaknite miš preko područja gdje bi widget trebao biti:

<div class="screenshot white-bg">
    <div class="title">Dodan widget s kodom</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodan widget s kodom" />
</div>

Sada povucite widget na željenu veličinu, vidjet ćete da se pojavljuje:

<div class="screenshot white-bg">
    <div class="title">Promijeni veličinu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Promijeni veličinu" />
</div>

...a sada pregledajte i spremite!