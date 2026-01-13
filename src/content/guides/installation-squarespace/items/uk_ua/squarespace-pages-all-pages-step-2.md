Тепер ми можемо скопіювати наведену нижче ділянку коду. Використайте кнопку копіювання, що з'являється у верхньому правому куті фрагмента.

Є кілька параметрів, які ви можете налаштувати в коді — див. рядки 4–7.

[inline-code-attrs-start title = 'Код коментарів для всіх сторінок Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...потім вставте у поле коду й натисніть зберегти. Це має виглядати так, з кодом у блоці `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Вставити та зберегти</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Вставити та зберегти" />
</div>

Якщо виникають проблеми, перевірте, щоб біля низу не було написано `"tenantId": "demo"`. Там має бути вказано ваш tenant id, якщо ви ввійшли в систему. Якщо ні — зверніться в підтримку.