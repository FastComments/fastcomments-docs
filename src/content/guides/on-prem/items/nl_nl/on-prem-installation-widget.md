De front-end codefragmenten en bibliotheken voor On-Prem zijn hetzelfde als het SaaS-product. Je moet echter `apiHost` en het juiste scriptpad opgeven:

[inline-code-attrs-start title = 'Reactiecode voor On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... kan ook SSO-payload enz. doorgeven.
    });
</script>
[inline-code-end]

Bovenstaand is een heel eenvoudig voorbeeld. We kunnen ook de officiële React-, Angular-, Vue-, Svelte- enz. bibliotheken gebruiken.

---