Виджет «Последние обсуждения» показывает страницы на вашем сайте, на которых была самая последняя активность комментариев. Каждая запись отображает заголовок страницы, дату последней активности и общее количество комментариев. Он автоматически определяет тёмные фоны и соответственно корректирует свои стили.

## Базовая установка

[inline-code-attrs-start title = 'Установка виджета последних обсуждений'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Параметры конфигурации

Функция `FastCommentsRecentDiscussionsV2` принимает следующие параметры конфигурации:

- **tenantId** (required): Ваш идентификатор tenant в FastComments
- **count** (optional): Количество страниц для отображения. По умолчанию `20`, максимум `100`
- **hasDarkBackground** (optional): Принудительно включить стиль тёмной темы. Если не задано, определяется автоматически по фону страницы

## Расширенные примеры

### Пользовательское количество

[inline-code-attrs-start title = 'Последние обсуждения с заданным количеством'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Принудительный тёмный режим

[inline-code-attrs-start title = 'Последние обсуждения в тёмном режиме'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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