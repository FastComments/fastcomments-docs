---
De front-end codefragmenten en bibliotheken voor On-Prem zijn hetzelfde als bij het SaaS-product. Je moet echter `apiHost` en het juiste scriptpad opgeven:

[inline-code-attrs-start title = 'Reactiescode voor On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... kan ook SSO-payload etc. doorgeven.
    }];
</script>
[inline-code-end]

Het bovenstaande is een zeer eenvoudig voorbeeld. We kunnen ook de officiële React-, Angular-, Vue-, Svelte- enz. bibliotheken gebruiken.

---