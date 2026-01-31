Теперь давайте добавим код нашего виджета.

Скопируйте код ниже. Убедитесь, что вы вошли в систему на [fastcomments.com](https://fastcomments.com) 
и перезагрузите эту страницу, если нет — тогда код будет предварительно заполнен информацией вашего аккаунта, в противном случае он покажет демонстрационный код.

Теперь скопируем код:

[inline-code-attrs-start title = 'Код комментариев Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Теперь вернёмся в конструктор сайта и нажмём `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Вставить код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Вставить код" />
</div>

### Примечание!

Важно использовать приведённый выше код, а не фрагменты кода из другой документации, так как этот фрагмент был специально подготовлен
для Hostinger.

Теперь у вас должно получиться что-то похожее на следующее, что выглядит пустым. Это ожидаемо. Подведите курсор мыши к области,
где должен появиться виджет:

<div class="screenshot white-bg">
    <div class="title">Виджет добавлен</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Виджет добавлен" />
</div>

Теперь перетащите виджет, чтобы задать нужный размер — вы увидите, как он появится:

<div class="screenshot white-bg">
    <div class="title">Изменить размер</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Изменить размер" />
</div>

...а теперь просмотрите и сохраните!