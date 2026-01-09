---
Фрагментите от фронтенд кода и библиотеките за On-Prem са същите като при SaaS продукта. Въпреки това, трябва да посочите `apiHost` и правилния път до скрипта:

[inline-code-attrs-start title = 'Код за коментари за On Prem'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://my.host.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        apiHost: "https://my.host.com"
        // ... може също да предадете SSO полезен товар и т.н.
    });
</script>
[inline-code-end]

Горният пример е много прост. Можем също да използваме официалните библиотеки за React, Angular, Vue, Svelte и т.н.

---