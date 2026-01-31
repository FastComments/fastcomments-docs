---
Теперь мы собираемся скопировать наш код. Если в фрагменте кода на строке 6 указано `tenantId: "demo"`, то вам следует войти в свою учетную запись FastComments
и затем обновить эту страницу, чтобы скопированный фрагмент кода содержал id вашей учетной записи.

[inline-code-attrs-start title = 'Фрагмент для Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Теперь вставьте его в редактор и нажмите Сохранить:

<div class="screenshot white-bg">
    <div class="title">Добавьте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Добавьте код FastComments" />
</div>

... затем сохраните сайт. Вот и всё!

---