Віджет Recent Discussions показує сторінки вашого сайту з найновішою активністю коментарів. Кожен запис відображає назву сторінки, дату останньої активності та загальну кількість коментарів. Він автоматично визначає темний фон і відповідно адаптує своє оформлення.

## Базове встановлення

[inline-code-attrs-start title = 'Встановлення віджета Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Параметри конфігурації

Функція `FastCommentsRecentDiscussionsV2` приймає такі параметри конфігурації:

- **tenantId** (обов\'язково): Ваш ідентифікатор tenant у FastComments
- **count** (необов\'язково): Кількість сторінок для показу. За замовчуванням `20`, максимум `100`
- **hasDarkBackground** (необов\'язково): Примусове оформлення темного режиму. Автоматично визначається з фону сторінки, якщо не задано

## Розширені приклади

### Власна кількість

[inline-code-attrs-start title = 'Recent Discussions з власною кількістю'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### Примусовий темний режим

[inline-code-attrs-start title = 'Recent Discussions з темним режимом'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]