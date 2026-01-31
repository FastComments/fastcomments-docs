Тепер додамо код нашого віджета.

Скопіюйте код нижче. Переконайтеся, що ви увійшли на [fastcomments.com](https://fastcomments.com) 
і поновіть цю сторінку, якщо ні, щоб код був попередньо заповнений інформацією вашого облікового запису; інакше буде показано демонстраційний код.

Тепер скопіюємо код:

[inline-code-attrs-start title = 'Код коментарів Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Поверніться до конструктора сайту й натисніть `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Ввести код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Ввести код" />
</div>

### Примітка!

Важливо використовувати наведенний вище код, а не фрагменти коду з інших розділів документації, оскільки цей фрагмент спеціально підготовлений
для Zyro.

Тепер у вас має з’явитися щось схоже на наступне, що виглядає порожнім. Це очікувано. Наведіть курсор миші на область, де має бути віджет:

<div class="screenshot white-bg">
    <div class="title">Віджет коду додано</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Віджет коду додано" />
</div>

Тепер перетягніть віджет до бажаного розміру — ви побачите, як він з’явиться:

<div class="screenshot white-bg">
    <div class="title">Змінити розмір</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Змінити розмір" />
</div>

...а тепер перегляньте та збережіть!