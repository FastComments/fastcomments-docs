---
Przykłady kodu front-end i biblioteki dla On-Prem są takie same jak w produkcie SaaS. Jednak musisz określić `apiHost` oraz poprawną ścieżkę do skryptu:

[inline-code-attrs-start title = 'Kod komentarzy dla On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... można również przekazać ładunek SSO itp.
    });
</script>
[inline-code-end]

Powyższy przykład jest bardzo prosty. Możemy też użyć oficjalnych bibliotek dla React, Angular, Vue, Svelte itp.

---