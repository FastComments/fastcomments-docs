Уиджетът „Скорошни дискусии“ показва страниците на вашия сайт с най-скорошна коментарна активност. Всеки запис показва заглавието на страницата, датата на последната активност и общия брой коментари. Той автоматично разпознава тъмен фон и съответно адаптира своя стил.

## Основна инсталация

[inline-code-attrs-start title = 'Инсталация на уиджет „Скорошни дискусии“'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Опции за конфигурация

Функцията `FastCommentsRecentDiscussionsV2` приема следните опции за конфигурация:

- **tenantId** (задължително): Вашият FastComments tenant ID
- **count** (по избор): Брой страници за показване. По подразбиране е `20`, максимум `100`
- **hasDarkBackground** (по избор): Принудително прилагане на тъмен режим. Автоматично се разпознава от фона на страницата, ако не е зададено

## Разширени примери

### Персонализиран брой

[inline-code-attrs-start title = 'Скорошни дискусии с персонализиран брой'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Принудителен тъмен режим

[inline-code-attrs-start title = 'Скорошни дискусии с тъмен режим'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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