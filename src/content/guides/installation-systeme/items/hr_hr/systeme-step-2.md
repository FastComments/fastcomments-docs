Sada ćemo kopirati naš kod. Ako u isječku koda na redu 6 piše `tenantId: "demo"` trebali biste se prijaviti na svoj FastComments račun i zatim osvježiti ovu stranicu kako bi kopirani isječak koda imao ID vašeg računa.

[inline-code-attrs-start title = 'Isječak Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Sada ga zalijepite u uređivač i kliknite spremi:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Dodajte FastComments kod" />
</div>

... zatim spremite svoju stranicu. To je to!