Тепер ми можемо скопіювати наступний фрагмент коду. Використайте кнопку копіювання, яка з'являється у верхньому правому куті фрагмента.

Є кілька налаштувань, які ви можете змінити в коді, див. рядки 4–7.

[inline-code-attrs-start title = 'Код для окремої сторінки Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // ваш ідентифікатор облікового запису
    }];
</script>
[inline-code-end]

Це має виглядати так:

<div class="screenshot white-bg">
    <div class="title">Вставте та збережіть</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Вставте та збережіть" />
</div>

Тепер натисніть Зберегти у верхньому правому куті.

Зауважте, що опція `Preview in Safe Mode` не працюватиме, але віджет з'явиться, коли ви відвідаєте свій сайт.

Якщо виникають проблеми, переконайтеся, що ближче до низу не вказано `"tenantId": "demo"`. Там має відображатися ваш tenant id, якщо ви увійшли в систему. Якщо ні, зверніться до служби підтримки.