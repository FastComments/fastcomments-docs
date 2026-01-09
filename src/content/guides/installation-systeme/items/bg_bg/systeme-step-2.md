Сега ще копираме нашия код. Ако фрагментът от кода казва `tenantId: "demo"` на ред 6 тогава трябва да влезете в своя акаунт в FastComments
и след това да презаредите тази страница така, че копираният фрагмент от кода да съдържа идентификатора на вашия акаунт.

[inline-code-attrs-start title = 'Фрагмент за Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Сега го поставете в редактора и натиснете 'Запазване':

<div class="screenshot white-bg">
    <div class="title">Добавете кода на FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Добавете кода на FastComments" />
</div>

... след това запазете сайта си. Това е всичко!