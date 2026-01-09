Тепер додамо код нашого віджета.

Скопіюйте код нижче. Переконайтеся, що ви ввійшли до облікового запису на [fastcomments.com](https://fastcomments.com)
і перезавантажте цю сторінку, якщо ні — тоді код буде попередньо заповнений інформацією вашого облікового запису, інакше буде показаний демонстраційний код.

Тепер скопіюємо код:

[inline-code-attrs-start title = 'Код коментарів Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Тепер повернімося до конструктора сайту і натисніть `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Вставити код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Вставити код" />
</div>

### Примітка!

Важливо використовувати наведенний вище код, а не фрагменти коду з іншої документації, оскільки цей фрагмент спеціально
підготовлений для Hostinger.

Тепер у вас має з'явитися щось на кшталт наступного, що виглядає порожнім. Це очікувано. Наведіть курсор миші на область,
де повинен бути віджет:

<div class="screenshot white-bg">
    <div class="title">Віджет коду додано</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Віджет коду додано" />
</div>

Тепер перетягніть віджет до потрібного розміру, він з'явиться:

<div class="screenshot white-bg">
    <div class="title">Змінити розмір</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Змінити розмір" />
</div>

...а тепер перегляньте та збережіть!