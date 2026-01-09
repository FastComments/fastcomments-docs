Тепер ми можемо скопіювати наступний фрагмент коду. Використайте кнопку копіювання, що з'являється у верхньому правому куті фрагмента.

У коді можна налаштувати кілька параметрів — див. рядки 4–7.

[inline-code-attrs-start title = 'Код для окремої сторінки Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // ваш ідентифікатор облікового запису

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Воно має виглядати так:

<div class="screenshot white-bg">
    <div class="title">Вставте та збережіть</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Вставте та збережіть" />
</div>

Тепер натисніть Зберегти у верхньому правому куті.

Зауважте, що опція `Preview in Safe Mode` не працюватиме, але віджет з'явиться, коли ви відвідаєте свій сайт.

Якщо у вас виникають проблеми, переконайтеся, що ближче до низу не вказано `"tenantId": "demo"`. Має відображатися ваш tenant id, якщо ви ввійшли в систему. Якщо ні — зверніться до служби підтримки.