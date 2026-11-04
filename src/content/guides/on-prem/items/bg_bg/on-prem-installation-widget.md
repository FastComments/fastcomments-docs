Кодовите фрагменти и библиотеки за предната част на On-Prem са същите като при SaaS продукта. Въпреки това трябва да посочите `apiHost` и правилния път до скрипта:

[inline-code-attrs-start title = 'Код за коментари за On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... can also pass SSO payload etc.
    }];
</script>
[inline-code-end]

Горното е много прост пример. Можем също да използваме първичните библиотеки за React, Angular, Vue, Svelte и др.