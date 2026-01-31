The front end code snippets and libraries for On-Prem are the same as the SaaS product. However, you must specify `apiHost` and the correct script path:

[inline-code-attrs-start title = 'Код за коментари за On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... може също да предадете SSO payload и т.н.
    }];
</script>
[inline-code-end]

The above is a very simple example. We could also use the 1st-party React, Angular, Vue, Svelte, etc, libraries.