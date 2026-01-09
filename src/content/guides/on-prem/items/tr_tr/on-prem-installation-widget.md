The front end code snippets and libraries for On-Prem are the same as the SaaS product. However, you must specify `apiHost` and the correct script path:

[inline-code-attrs-start title = 'On-Prem İçin Yorum Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... ayrıca SSO payload vb. de geçebilirsiniz.
    });
</script>
[inline-code-end]

The above is a very simple example. We could also use the 1st-party React, Angular, Vue, Svelte, etc, libraries.

---