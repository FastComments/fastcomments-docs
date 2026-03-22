Виџет Недавне дискусије приказује странице на вашем сајту које имају најновију активност коментара. Сваки унос приказује наслов странице, датум последње активности и укупни број коментара. Аутоматски детектује тамне позадине и прилагођава свој стил у складу с тим.

## Основна инсталација

[inline-code-attrs-start title = 'Инсталација виџета Недавне дискусије'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
- **count** (опционо): Број страница које ће бити приказане. Подразумевано је `20`, максимум `100`
- **hasDarkBackground** (опционо): Присили стил тамног режима. Ако није подешено, аутоматски се детектује из позадине странице

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

### Принудни тамни режим

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