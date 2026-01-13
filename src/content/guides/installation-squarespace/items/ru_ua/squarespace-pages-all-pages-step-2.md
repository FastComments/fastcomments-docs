Теперь мы можем скопировать следующий фрагмент кода. Используйте кнопку копирования, которая появляется в правом верхнем углу фрагмента.

Есть несколько параметров, которые вы можете настроить в коде — смотрите строки 4–7.

[inline-code-attrs-start title = 'Код комментариев для всех страниц Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш идентификатор аккаунта

        function tryLoad() {
            // попытка загрузить для разных макетов
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

...затем вставьте в область кода и нажмите сохранить. Это должно выглядеть так, с кодом в блоке `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Вставьте и сохраните</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Вставьте и сохраните" />
</div>

Если у вас возникают проблемы, убедитесь, что внизу не написано "tenantId": "demo". Должен отображаться ваш tenant id, если вы вошли в систему. Если нет — обратитесь в службу поддержки.