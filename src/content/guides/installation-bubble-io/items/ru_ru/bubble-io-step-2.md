Нажмите на только что добавленный HTML-элемент. В появившемся редакторе свойств вставьте следующий код в поле HTML:

[inline-code-attrs-start title = 'Фрагмент кода для живых комментариев Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble часто меняет фрагмент кода на асинхронный
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
    <div class="title">Вставка кода FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Вставка кода FastComments в HTML-элемент" />
</div>

Примечание: этот код включает механизм повторных попыток, чтобы гарантировать корректную загрузку FastComments в динамичной среде Bubble. Другие фрагменты кода работать не будут.

Не забудьте заменить `'demo'` на ваш реальный FastComments tenant ID после регистрации. Если вы вошли в FastComments.com, замена должна быть выполнена автоматически.