Front end-kodeuddragene og bibliotekerne til On-Prem er de samme som SaaS-produktet. Du skal dog angive `apiHost` og den korrekte scriptsti:

[inline-code-attrs-start title = 'Kode til kommentarer for On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... kan også sende SSO-payload osv.
    });
</script>
[inline-code-end]

Ovenstående er et meget simpelt eksempel. Vi kan også bruge førsteparts React-, Angular-, Vue-, Svelte- osv. biblioteker.