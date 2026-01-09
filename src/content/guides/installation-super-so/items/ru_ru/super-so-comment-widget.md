---
## Добавление виджета живых комментариев в ваши статьи Notion на Super.so

В дополнение к Collab Chat, вы можете добавить традиционный виджет комментариев внизу ваших статей Notion. Это позволяет читателям оставлять комментарии и обсуждать всю статью.

### Шаги установки

Скопируйте следующий код и вставьте его в раздел `Body` настроек вашего сайта Super.so:

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

            // Очистка существующего экземпляра
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Создать новый целевой элемент
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Инициализация FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Обновить текущий pathname
            currentPathname = window.location.pathname;
        }

        // Начальная загрузка
        load();

        // Проверять изменения каждые 500 мс
        setInterval(() => {
            // Перезагрузить, если pathname изменился
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Перезагрузить, если виджет был удалён
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Перезагрузить, если контейнер опустел
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
- Для каждой страницы создаётся собственная уникальная ветка комментариев на основе пути URL
- Обязательно замените `"demo"` на ваш реальный tenant ID из аккаунта FastComments
- Виджет автоматически обрабатывает динамическую загрузку страниц Super.so

---