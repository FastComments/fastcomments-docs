---
Die Frontend-Code-Snippets und Bibliotheken für On-Prem sind dieselben wie beim SaaS-Produkt. Sie müssen jedoch `apiHost` und den korrekten Skriptpfad angeben:

[inline-code-attrs-start title = 'Kommentare-Code für On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... es kann auch SSO-Payload usw. übergeben werden.
    });
</script>
[inline-code-end]

Das Obige ist ein sehr einfaches Beispiel. Wir könnten auch die First-Party-Bibliotheken für React, Angular, Vue, Svelte usw. verwenden.

---