Теперь давайте добавим код виджета.

Скопируйте код ниже. Убедитесь, что вы вошли в систему на [fastcomments.com](https://fastcomments.com) 
и перезагрузите эту страницу, если нет, чтобы код был предварительно заполнен информацией вашего аккаунта, в противном случае будет показан демонстрационный код.

Теперь скопируем код:

[inline-code-attrs-start title = 'Код комментариев Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Теперь вернитесь в конструктор сайта и нажмите `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Enter code</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Ввести код" />
</div>

### Примечание!

Важно использовать приведённый выше код, а не фрагменты из другой документации, так как этот фрагмент был специально подготовлен для Zyro.

Теперь у вас должно получиться нечто подобное, что выглядит пустым. Это ожидаемо. Наведите курсор мыши на область,
где должен быть виджет:

<div class="screenshot white-bg">
    <div class="title">Код виджета добавлен</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Код виджета добавлен" />
</div>

Теперь перетащите виджет, задав желаемый размер, вы увидите, как он появится:

<div class="screenshot white-bg">
    <div class="title">Измените размер</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Измените размер" />
</div>

...а теперь просмотрите и сохраните!

---