Клацніть на HTML-елементі, який ви щойно додали. У редакторі властивостей, що з'явиться, вставте наступний код у поле HTML:

[inline-code-attrs-start title = 'Код вставки живих коментарів Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble часто змінює фрагмент коду на async
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
    <div class="title">Вставте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Вставлення коду FastComments у HTML-елемент" />
</div>

Примітка: цей код містить механізм повторних спроб, щоб гарантувати правильне завантаження FastComments у динамічному середовищі Bubble. Інші фрагменти коду не працюватимуть.

Пам'ятайте замінити `'demo'` на ваш фактичний FastComments tenant ID після реєстрації. Якщо ви ввійшли в FastComments.com, він має бути вже замінений.