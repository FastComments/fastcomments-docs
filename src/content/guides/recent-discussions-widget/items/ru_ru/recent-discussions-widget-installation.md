Виджет «Последние обсуждения» показывает страницы на вашем сайте с самой последней активностью комментариев. Каждая запись отображает заголовок страницы, дату последней активности и общее число комментариев. Виджет автоматически определяет тёмный фон и соответствующим образом корректирует стиль.

## Базовая установка

[inline-code-attrs-start title = 'Установка виджета "Последние обсуждения"'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

- **tenantId** (обязательно): Ваш идентификатор tenant в FastComments
- **count** (необязательно): Количество отображаемых страниц. По умолчанию `20`, максимум `100`
- **hasDarkBackground** (необязательно): Принудительно включать оформление в тёмном режиме. Если не задано, определяется автоматически по фону страницы

## Продвинутые примеры

### Пользовательское количество

[inline-code-attrs-start title = 'Последние обсуждения с настраиваемым количеством'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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