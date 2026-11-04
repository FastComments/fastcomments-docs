Теперь мы можем скопировать следующий фрагмент кода. Используйте кнопку копирования, которая появляется в правом верхнем углу фрагмента.

В коде можно настроить несколько параметров, см. строки с 4 по 7.

[inline-code-attrs-start title = 'Код комментариев для всех страниц Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // идентификатор вашего аккаунта

        function tryLoad() {
            // пробуем загрузить для разных макетов
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...затем вставьте код в область кода и нажмите сохранить. Это должно выглядеть так, с кодом в блоке `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Вставка и сохранение</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Вставка и сохранение" />
</div>

Если у вас возникли проблемы, убедитесь, что внизу не указано `"tenantId": "demo"`. Если вы вошли в систему, должен отображаться ваш идентификатор tenant. Если нет, обратитесь в поддержку.