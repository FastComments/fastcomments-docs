Sada ćemo kopirati naš kod. Ako isječak koda kaže `tenantId: "demo"` na liniji 6 onda se trebate prijaviti na svoj FastComments račun
i zatim osvježiti ovu stranicu tako da kopirani isječak koda sadrži ID vašeg računa.

[inline-code-attrs-start title = 'Isječak Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Sada ga zalijepite u uređivač i kliknite Spremi:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Dodajte FastComments kod" />
</div>

... zatim spremite svoju stranicu. To je to!