Тепер ми скопіюємо наш код. Якщо у фрагменті коду на рядку 6 зазначено `tenantId: "demo"`, то вам слід увійти в свій обліковий запис FastComments
і потім оновити цю сторінку, щоб скопійований фрагмент коду містив ID вашого облікового запису.

[inline-code-attrs-start title = 'Фрагмент Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Тепер вставте його в редактор і натисніть Зберегти:

<div class="screenshot white-bg">
    <div class="title">Додайте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Додайте код FastComments" />
</div>

... потім збережіть свій сайт. Ось і все!