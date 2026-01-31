Фрагменти коду фронтенду та бібліотеки для On‑Prem такі ж, як у SaaS-продукту. Однак потрібно вказати `apiHost` та правильний шлях до скрипта:

[inline-code-attrs-start title = 'Код коментарів для On-Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://my.host.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... можна також передати SSO payload тощо.
    }];
</script>
[inline-code-end]

Наведене вище — дуже простий приклад. Ми також можемо використовувати офіційні бібліотеки для React, Angular, Vue, Svelte тощо.