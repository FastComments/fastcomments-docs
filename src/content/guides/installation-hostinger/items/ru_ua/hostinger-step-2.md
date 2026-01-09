Теперь давайте добавим код нашего виджета.

Скопируйте код ниже. Убедитесь, что вы вошли в систему на [fastcomments.com](https://fastcomments.com) 
и перезагрузите эту страницу, если нет — так код будет предварительно заполнен информацией вашего аккаунта, в противном случае будет показан демонстрационный код.

Теперь скопируем код:

[inline-code-attrs-start title = 'Код комментариев Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Теперь вернёмся к нашему конструктору сайта и нажмём `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Вставить код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Вставить код" />
</div>

### Примечание!

Важно использовать указанный выше код, а не фрагменты из другой документации, поскольку этот фрагмент специально подготовлен для Hostinger.

Теперь у вас должно быть что-то похожее на следующее, что выглядит пустым. Это ожидаемо. Наведите курсор мыши на область, где должен отображаться виджет:

<div class="screenshot white-bg">
    <div class="title">Виджет добавлен</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Виджет добавлен" />
</div>

Теперь перетащите виджет до нужного размера, вы увидите, как он появится:

<div class="screenshot white-bg">
    <div class="title">Измените размер</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Измените размер" />
</div>

...а теперь просмотрите и сохраните!