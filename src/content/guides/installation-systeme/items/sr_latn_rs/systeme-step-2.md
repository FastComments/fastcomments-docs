Sada ćemo kopirati naš kod. Ako u isječku koda na liniji 6 piše `tenantId: "demo"` onda treba da se prijavite na vaš FastComments nalog
i potom osvežite ovu stranicu tako da kopirani isječak koda ima ID vašeg naloga.

[inline-code-attrs-start title = 'Isečak za Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

Sada ga nalepite u editor i kliknite sačuvaj:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Dodajte FastComments kod" />
</div>

... zatim sačuvajte svoj sajt. To je to!