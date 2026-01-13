---
Тепер ми можемо скопіювати наступний фрагмент коду (використайте кнопку копіювання у верхньому правому куті фрагмента):

[inline-code-attrs-start title = 'Код коментарів блогу Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш ідентифікатор облікового запису

        function tryLoad() {
            // спробувати завантажити для різних макетів
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

...потім вставте в поле коду та натисніть зберегти:

<div class="screenshot white-bg">
    <div class="title">Вставити та зберегти</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Вставити та зберегти" />
</div>

---