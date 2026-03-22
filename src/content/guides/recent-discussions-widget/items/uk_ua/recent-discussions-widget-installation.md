The Recent Discussions Widget shows pages on your site that have the most recent comment activity. Each entry displays the page title, last activity date, and total comment count. It automatically detects dark backgrounds and adjusts its styling accordingly.

## Basic Installation

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

## Configuration Options

The `FastCommentsRecentDiscussionsV2` function accepts the following configuration options:

- **tenantId** (required): Ваш ідентифікатор орендаря FastComments
- **count** (optional): Кількість сторінок для відображення. За замовчуванням `20`, максимум `100`
- **hasDarkBackground** (optional): Примусово вмикає темний режим стилів. Якщо не встановлено, визначається автоматично за фоном сторінки

## Advanced Examples

### Custom Count

[inline-code-attrs-start title = 'Recent Discussions з налаштованою кількістю'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Force Dark Mode

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

---