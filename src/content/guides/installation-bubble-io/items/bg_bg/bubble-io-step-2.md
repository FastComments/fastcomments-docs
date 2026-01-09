Кликнете върху HTML елемента, който току-що добавихте. В редактора на свойства, който ще се появи, поставете следния код в полето HTML:

[inline-code-attrs-start title = 'Кодов фрагмент за живо коментиране в Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // bubble има склонност да променя фрагмента с код да стане асинхронен
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
    <div class="title">Вмъкване на кода на FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Вмъкване на кода на FastComments в HTML елемент" />
</div>

Забележка: Този код включва механизъм за повторен опит, за да се гарантира, че FastComments се зарежда правилно в динамичната среда на Bubble. Други фрагменти с код няма да работят.

Не забравяйте да замените 'demo' с вашия реален FastComments tenant ID след като се регистрирате. Ако сте влезли в FastComments.com, той вече трябва да е заменен.