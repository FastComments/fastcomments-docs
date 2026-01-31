Die Frontend-Code-Snippets und Bibliotheken für On-Prem sind die gleichen wie beim SaaS-Produkt. Sie müssen jedoch `apiHost` und den richtigen Skriptpfad angeben:

[inline-code-attrs-start title = 'Kommentare-Code für On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... kann auch SSO-Payload usw. übergeben werden.
    }];
</script>
[inline-code-end]

Das obige ist ein sehr einfaches Beispiel. Wir könnten auch die Erstanbieter-Bibliotheken für React, Angular, Vue, Svelte usw. verwenden.