Виџет Недавних дискусија приказује странице на вашем сајту које имају најскорију активност коментара. Сваки унос приказује наслов странице, датум последње активности и укупан број коментара. Аутоматски детектује тамне позадине и прилагођава свој стил сходно томе.

## Основна инсталација

[inline-code-attrs-start title = 'Инсталација виџета Недавних дискусија'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

Функција `FastCommentsRecentDiscussionsV2` прихвата следеће опције конфигурације:

- **tenantId** (обавезно): Ваш FastComments tenant ID
- **count** (опционо): Број страница за приказивање. Подразумевано је `20`, максимум `100`
- **hasDarkBackground** (опционо): Форсира стил тамног режима. Аутоматски се детектује из позадине странице ако није постављено

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

[inline-code-attrs-start title = 'Недавне дискусије са тамним режимом'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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