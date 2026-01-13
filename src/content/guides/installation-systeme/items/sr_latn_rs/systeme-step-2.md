Sada ćemo kopirati naš kod. Ako isječak koda na liniji 6 prikazuje `tenantId: "demo"`, prijavite se na svoj FastComments nalog
i zatim osvežite ovu stranicu tako da kopirani isječak koda sadrži ID vašeg naloga.

[inline-code-attrs-start title = 'Isečak za Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Sada ga nalepite u editor i kliknite sačuvaj:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Dodajte FastComments kod" />
</div>

... zatim sačuvajte vaš sajt. To je to!