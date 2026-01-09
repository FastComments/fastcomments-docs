## Додавання віджета живих коментарів до ваших статей Notion на Super.so

На додаток до Collab Chat, ви можете додати традиційний віджет коментарів внизу ваших статей Notion. Це дозволяє читачам залишати коментарі та вести обговорення щодо всієї статті.

### Кроки встановлення

Скопіюйте наведений код і вставте його в секцію `Body` налаштувань вашого сайту Super.so:

[inline-code-attrs-start title = 'Віджет живих коментарів FastComments для Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Очистити наявний екземпляр
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Створити нову ціль
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Ініціалізувати FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Оновити поточний pathname
            currentPathname = window.location.pathname;
        }

        // Початкове завантаження
        load();

        // Перевіряти на зміни кожні 500 мс
        setInterval(() => {
            // Перезавантажити, якщо pathname змінився
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Перезавантажити, якщо віджет було видалено
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Перезавантажити, якщо контейнер спорожнів
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Важливі примітки

- Віджет коментарів з'явиться внизу ваших статей Notion
- Кожна сторінка отримає власну унікальну гілку коментарів на основі шляху URL
- Переконайтеся, що замінили `"demo"` на ваш фактичний tenant ID з вашого облікового запису FastComments
- Віджет автоматично обробляє динамічне завантаження сторінок Super.so

---