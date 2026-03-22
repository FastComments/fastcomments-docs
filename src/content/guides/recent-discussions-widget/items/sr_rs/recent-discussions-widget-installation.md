The Recent Discussions Widget shows pages on your site that have the most recent comment activity. Each entry displays the page title, last activity date, and total comment count. It automatically detects dark backgrounds and adjusts its styling accordingly.

## Основна инсталација

[inline-code-attrs-start title = 'Инсталација видгета Недавне дискусије'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Опције конфигурације

The `FastCommentsRecentDiscussionsV2` function accepts the following configuration options:

- **tenantId** (required): Ваш FastComments tenant ID
- **count** (optional): Број страница за приказ. Подразумевано је `20`, максимум `100`
- **hasDarkBackground** (optional): Приморава стил тамног режима. Ако није подешено, аутоматски се детектује из позадине странице

## Напредни примери

### Прилагођени број

[inline-code-attrs-start title = 'Недавне дискусије са прилагођеним бројем'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Форсирање тамног режима

[inline-code-attrs-start title = 'Недавне дискусије у тамном режиму'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

---