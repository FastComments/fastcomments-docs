Этот пример использует пользовательский код для совместимости с Wix. **Вы не сможете использовать фрагменты кода FastComments из других руководств.**

Откройте форму для добавления нашего пользовательского HTML-диалога, нажав `Enter Code` и выбрав `HTML`:

<div class="screenshot white-bg">
    <div class="title">Шаг 3: Открыть HTML-диалог</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Шаг 3: Открыть HTML-диалог" />
</div>

Скопируйте следующий HTML-фрагмент и вставьте его в диалог, затем нажмите «Обновить»:

[inline-code-attrs-start title = 'Фрагмент кода комментариев для Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Шаг 3: Вставить и сохранить</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Шаг 3: Вставить и сохранить" />
</div>

Теперь вы должны увидеть очень маленькую версию виджета комментариев:

<div class="screenshot white-bg">
    <div class="title">Шаг 3: Результат</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Шаг 3: Результат" />
</div>

Далее мы позиционируем и изменим размер виджета, чтобы он вписался в нашу страницу.

---