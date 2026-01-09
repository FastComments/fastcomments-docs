Теперь мы можем скопировать следующий фрагмент кода. Используйте кнопку копирования, которая появляется в правом верхнем углу фрагмента.

Есть несколько параметров, которые вы можете настроить в коде — см. строки 4–7.

[inline-code-attrs-start title = 'Код для отдельной страницы Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш идентификатор аккаунта

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Это должно выглядеть так:

<div class="screenshot white-bg">
    <div class="title">Вставьте и сохраните</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Вставьте и сохраните" />
</div>

Теперь нажмите «Сохранить» в правом верхнем углу.

Обратите внимание, что опция `Preview in Safe Mode` не будет работать, но виджет появится, когда вы посетите свой сайт.

Если у вас возникают проблемы, убедитесь, что внизу не указано `"tenantId": "demo"`. Должен отображаться ваш tenant id, если вы вошли в систему. Если нет — обратитесь в службу поддержки.