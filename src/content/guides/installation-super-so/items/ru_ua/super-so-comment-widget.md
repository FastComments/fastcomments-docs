## Добавление виджета живых комментариев в ваши статьи Notion на Super.so

В дополнение к Collab Chat вы можете добавить традиционный виджет комментариев внизу ваших статей Notion. Это позволяет читателям оставлять комментарии и вести обсуждения по всей статье.

### Шаги установки

Скопируйте следующий код и вставьте его в секцию `Body` настроек вашего сайта Super.so:

[inline-code-attrs-start title = 'Виджет живых комментариев FastComments для Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Удаляем существующий экземпляр
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Создаем новый контейнер
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Инициализируем FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Обновляем текущий путь
            currentPathname = window.location.pathname;
        }

        // Первичная загрузка
        load();

        // Проверяем изменения каждые 500 мс
        setInterval(() => {
            // Перезагружаем, если путь изменился
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Перезагружаем, если виджет был удален
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Перезагружаем, если контейнер опустел
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Важные примечания

- Виджет комментариев появится внизу ваших статей Notion
- Каждая страница получает свою уникальную ветку комментариев на основе пути URL
- Убедитесь, что заменили "demo" на ваш реальный tenant ID из учетной записи FastComments
- Виджет автоматически обрабатывает динамическую загрузку страниц Super.so

---