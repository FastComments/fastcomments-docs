Тепер додамо код нашого віджета.

Скопіюйте код нижче. Переконайтеся, що ви увійшли в обліковий запис на [fastcomments.com](https://fastcomments.com) 
і перезавантажте цю сторінку, якщо ні, щоб код був попередньо заповнений інформацією вашого акаунту, інакше він покаже демонстраційний код.

Тепер скопіюємо код:

[inline-code-attrs-start title = 'Код коментарів Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Тепер поверніться до конструктора сайту та натисніть `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Вставити код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Вставити код" />
</div>

### Примітка!

Важливо використовувати вищенаведений код, а не фрагменти з іншої документації, оскільки цей фрагмент спеціально підготовлений для Hostinger.

Тепер у вас має бути щось на кшталт наведеного нижче, що виглядає порожнім. Це очікувано. Наведіть курсор миші на область, де має з'явитися віджет:

<div class="screenshot white-bg">
    <div class="title">Віджет додано</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Віджет додано" />
</div>

Тепер перетягніть віджет до потрібного розміру — ви побачите, як він з'явиться:

<div class="screenshot white-bg">
    <div class="title">Змінити розмір</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Змінити розмір" />
</div>

...а тепер перегляньте та збережіть!