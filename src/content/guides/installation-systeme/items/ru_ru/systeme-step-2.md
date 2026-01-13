Теперь мы собираемся скопировать наш код. Если в фрагменте кода на строке 6 указано `tenantId: "demo"`, войдите в свою учетную запись FastComments, а затем обновите эту страницу, чтобы скопированный фрагмент кода содержал идентификатор вашей учетной записи.

[inline-code-attrs-start title = 'Сниппет Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Вставьте его в редактор и нажмите сохранить:

<div class="screenshot white-bg">
    <div class="title">Добавьте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Добавьте код FastComments" />
</div>

... затем сохраните сайт. Вот и всё!