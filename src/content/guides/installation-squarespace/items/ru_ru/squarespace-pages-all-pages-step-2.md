Теперь мы можем скопировать следующий фрагмент кода. Используйте кнопку копирования, которая появляется в правом верхнем углу фрагмента.

There are a few things you can configure in the code, see lines 4 through 7.

[inline-code-attrs-start title = 'Код комментариев для всех страниц Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш идентификатор аккаунта

        function tryLoad() {
            // попытка загрузки для разных макетов
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

...then paste in the code area and click save. It should look like this, with the code in the `FOOTER` block:

<div class="screenshot white-bg">
    <div class="title">Вставьте и сохраните</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Вставьте и сохраните" />
</div>

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.