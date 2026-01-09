На следующем шаге вам нужно скопировать заранее подготовленный код виджета ниже.

Пока вы авторизованы на FastComments.com, в приведённом ниже фрагменте кода уже будут данные вашего аккаунта. Скопируем его:

[inline-code-attrs-start title = 'Код FastComments Collab Chat для Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Очистить существующий экземпляр
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Удалить существующую верхнюю панель, если она есть
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Создать новую верхнюю панель
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Инициализация FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Обновить currentPathname
            currentPathname = window.location.pathname;
        }

        // Первичная загрузка
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

            // Перезагрузить, если контейнер был опустошён
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Теперь вставьте в область `Body`:

<div class="screenshot white-bg">
    <div class="title">Вставленный код</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Вставленный код" />
</div>

Если после вставки кода вы увидите сообщение "this is a demo message":

- Убедитесь, что вы вошли в свою учётную запись на fastcomments.com.
- Убедитесь, что у вас включены сторонние cookie.
- Затем обновите эту страницу и снова скопируйте фрагмент кода. В нём поле `tenantId` должно быть заполнено идентификатором вашего тенанта.