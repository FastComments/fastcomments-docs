---
Фрагменты кода фронтенда и библиотеки для On-Prem такие же, как и в SaaS-продукте. Однако вы должны указать `apiHost` и правильный путь к скрипту:

[inline-code-attrs-start title = 'Код комментариев для On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... можно также передать SSO payload и т.п.
    });
</script>
[inline-code-end]

Приведённый выше пример очень простой. Мы также можем использовать официальные библиотеки для React, Angular, Vue, Svelte и т.д.

---