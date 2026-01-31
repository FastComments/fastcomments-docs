Фрагменты кода фронтенда и библиотеки для On-Prem такие же, как у SaaS-продукта. Однако необходимо указать `apiHost` и корректный путь к скрипту:

[inline-code-attrs-start title = 'Код комментариев для On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... также можно передать полезную нагрузку SSO и т.д.
    }];
</script>
[inline-code-end]

Приведённый выше пример очень простой. Мы также можем использовать официальные библиотеки для React, Angular, Vue, Svelte и т.д.