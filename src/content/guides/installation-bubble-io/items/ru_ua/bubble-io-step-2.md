---
Нажмите на HTML-элемент, который вы только что добавили. В появившемся редакторе свойств вставьте следующий код в поле HTML:

[inline-code-attrs-start title = 'Фрагмент кода для живых комментариев Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble, как правило, изменяет фрагмент кода, делая его async
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Вставьте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Вставка кода FastComments в HTML-элемент" />
</div>

Примечание: этот код включает механизм повторных попыток, чтобы обеспечить корректную загрузку FastComments в динамической среде Bubble.
Другие фрагменты кода работать не будут.

Не забудьте заменить 'demo' на ваш реальный FastComments tenant ID после регистрации. Если вы вошли на FastComments.com, 'demo' уже должен быть заменён.

---