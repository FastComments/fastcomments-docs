Теперь мы можем скопировать следующий фрагмент кода. Используйте кнопку копирования, которая появляется в правом верхнем углу фрагмента.

Есть несколько параметров, которые вы можете настроить в коде — смотрите строки 4–7.

[inline-code-attrs-start title = 'Код для отдельной страницы Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // ваш идентификатор аккаунта
    }];
</script>
[inline-code-end]

Это должно выглядеть так:

<div class="screenshot white-bg">
    <div class="title">Paste and Save</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Paste and Save" />
</div>

Теперь нажмите «Сохранить» в правом верхнем углу.

Обратите внимание, что опция `Preview in Safe Mode` не будет работать, но виджет появится, когда вы посетите свой сайт.

Если у вас возникают проблемы, убедитесь, что в нижней части не указано `"tenantId": "demo"`. Там должен отображаться ваш tenant id, если вы вошли в систему. Если нет — обратитесь в службу поддержки.