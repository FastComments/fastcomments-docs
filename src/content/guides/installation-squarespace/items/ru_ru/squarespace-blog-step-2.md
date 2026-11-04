Теперь мы можем скопировать следующий фрагмент кода (используйте кнопку копирования в правом верхнем углу фрагмента):

[inline-code-attrs-start title = 'Код комментариев блога Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...затем вставьте код в область кода и нажмите сохранить:

<div class="screenshot white-bg">
    <div class="title">Вставьте и сохраните</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Вставьте и сохраните" />
</div>