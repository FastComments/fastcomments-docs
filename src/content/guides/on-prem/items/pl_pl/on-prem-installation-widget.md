Fragmenty kodu front-end i biblioteki dla On-Prem są takie same jak w produkcie SaaS. Jednak musisz określić `apiHost` oraz poprawną ścieżkę skryptu:

[inline-code-attrs-start title = 'Kod komentarzy dla On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... można też przekazać dane SSO itp.
    }];
</script>
[inline-code-end]

Powyższy przykład jest bardzo prosty. Możemy też użyć oficjalnych bibliotek React, Angular, Vue, Svelte itp.

---