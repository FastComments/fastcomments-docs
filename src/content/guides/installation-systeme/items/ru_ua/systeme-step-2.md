---
Теперь мы скопируем наш код. Если в фрагменте кода на строке 6 указано `tenantId: "demo"` то вам следует войти в свой аккаунт FastComments
и затем обновить эту страницу, чтобы в скопированном фрагменте кода был ваш идентификатор аккаунта.

[inline-code-attrs-start title = 'Фрагмент кода Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Теперь вставьте его в редактор и нажмите сохранить:

<div class="screenshot white-bg">
    <div class="title">Добавьте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Добавьте код FastComments" />
</div>

... затем сохраните свой сайт. На этом всё!

---