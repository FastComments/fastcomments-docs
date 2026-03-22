Видџет Recent Discussions приказује странице на вашем сајту које имају најновију активност коментара. Сваки унос приказује наслов странице, датум последње активности и укупни број коментара. Аутоматски открива тамну позадину и у складу с тим прилагођава свој стил.

## Osnovna instalacija

[inline-code-attrs-start title = 'Инсталација видџета Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## Opcije konfiguracije

Funkcija `FastCommentsRecentDiscussionsV2` прихвата следеће опције конфигурације:

- **tenantId** (обавезно): Ваш FastComments tenant ID
- **count** (опционо): Број страница за приказ. Подразумевано је `20`, максимум `100`
- **hasDarkBackground** (опционо): Приморати тамни режим стилизације. Ако није постављено, аутоматски се открива из позадине странице

## Напредни примјери

### Прилагођени број

[inline-code-attrs-start title = 'Recent Discussions са прилагођеним бројем'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Recent Discussions са тамним режимом'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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