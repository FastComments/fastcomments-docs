Теперь добавим код нашего виджета.

Скопируйте код ниже. Убедитесь, что вы вошли в аккаунт на [fastcomments.com](https://fastcomments.com) 
и перезагрузите эту страницу, если нет — тогда код будет предварительно заполнен информацией вашего аккаунта, иначе будет показан демонстрационный код.

Теперь скопируем код:

[inline-code-attrs-start title = 'Код комментариев Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Теперь вернитесь в наш конструктор сайта и нажмите `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Ввести код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Ввести код" />
</div>

### Важно!

Важно использовать приведённый выше код, а не фрагменты кода из другой документации, поскольку этот фрагмент специально подготовлен для Zyro.

Теперь у вас должно быть что-то похожее на следующее, что выглядит пустым. Это ожидаемо. Наведите курсор мыши на область,
где должен быть виджет:

<div class="screenshot white-bg">
    <div class="title">Виджет добавлен</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Виджет добавлен" />
</div>

Теперь перетащите виджет до нужного размера — вы увидите, как он появится:

<div class="screenshot white-bg">
    <div class="title">Изменить размер</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Изменить размер" />
</div>

...а теперь предпросмотрите и сохраните!