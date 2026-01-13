Тепер ми скопіюємо наш код. Якщо в кодовому фрагменті на рядку 6 написано `tenantId: "demo"` тоді вам слід увійти у свій обліковий запис FastComments
і потім оновити цю сторінку, щоб скопійований фрагмент коду містив ідентифікатор вашого акаунта.

[inline-code-attrs-start title = 'Фрагмент Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Тепер вставте його в редактор і натисніть зберегти:

<div class="screenshot white-bg">
    <div class="title">Додайте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Додайте код FastComments" />
</div>

... потім збережіть свій сайт. Ось і все!