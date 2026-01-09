Фрагменти коду фронтенду та бібліотеки для On-Prem такі ж, як і для SaaS-продукту. Однак ви повинні вказати `apiHost` та правильний шлях до скрипта:

[inline-code-attrs-start title = 'Код коментарів для On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... can also pass SSO payload etc.
    });
</script>
[inline-code-end]

Вищенаведений приклад — дуже простий. Ми також можемо використовувати офіційні бібліотеки для React, Angular, Vue, Svelte тощо.