Не се препоръчва да добавяте FastComments чрез Page Builder на BigCommerce, тъй като тогава кодът трябва да бъде ръчно добавен на всяка желана страница.

Въпреки това, ако това е желателно, следният кодов фрагмент трябва да се използва. Кодови фрагменти от други уроци няма да работят поради спецификата на BigCommerce:

[inline-code-attrs-start title = 'Фрагмент за Page Builder на BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

---