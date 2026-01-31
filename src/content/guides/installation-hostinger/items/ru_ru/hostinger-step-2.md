Теперь давайте добавим код виджета.

Скопируйте код ниже. Убедитесь, что вы вошли в систему на [fastcomments.com](https://fastcomments.com) 
и перезагрузите эту страницу, если нет, чтобы код был предварительно заполнен информацией вашей учётной записи, в противном случае будет показан демонстрационный код.

Теперь скопируйте код:

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

Теперь вернёмся к конструктору сайта и нажмите `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Вставить код</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Вставить код" />
</div>

### Примечание!

Важно использовать приведённый выше код, а не фрагменты кода из другой документации, поскольку этот сниппет был специально подготовлен для Hostinger.

Теперь у вас должно быть нечто подобное, что выглядит пустым. Это ожидаемо. Наведите курсор мыши на область
где должен быть виджет:

<div class="screenshot white-bg">
    <div class="title">Виджет кода добавлен</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Виджет кода добавлен" />
</div>

Теперь перетащите виджет до нужного размера — вы увидите, как он появится:

<div class="screenshot white-bg">
    <div class="title">Изменить размер</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Изменить размер" />
</div>

...а теперь предварительно просмотрите и сохраните!