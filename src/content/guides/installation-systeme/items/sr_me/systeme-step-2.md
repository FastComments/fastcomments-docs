---
Sada ćemo kopirati naš kod. Ako u isječku koda na liniji 6 piše `tenantId: "demo"`, trebalo bi da se prijavite na svoj FastComments nalog
i potom osvježite ovu stranicu da bi kopirani isječak koda sadržao ID vašeg naloga.

[inline-code-attrs-start title = 'Isječak za Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Sada ga zalijepite u editor i kliknite sačuvaj:

<div class="screenshot white-bg">
    <div class="title">Dodajte FastComments kod</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Dodajte FastComments kod" />
</div>

... zatim sačuvajte svoj sajt. To je to!

---