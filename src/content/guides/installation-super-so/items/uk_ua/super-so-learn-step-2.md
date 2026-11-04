На наступному кроці вам потрібно скопіювати наведений нижче готовий код віджета.

Оскільки ви увійшли на FastComments.com, наведений нижче фрагмент коду вже міститиме інформацію про ваш обліковий запис. Давайте скопіюємо його:

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

            // Очистити наявний екземпляр
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Очистити наявну верхню панель, якщо вона існує
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Створити нову верхню панель
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Ініціалізувати FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Оновити поточний pathname
            currentPathname = window.location.pathname;
        }

        // Початкове завантаження
        load();

        // Перевіряти зміни кожні 500 мс
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

            // Перезавантажити, якщо контейнер було очищено
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Тепер вставте код у область `Body`:

<div class="screenshot white-bg">
    <div class="title">Вставлений код</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Вставлений код" />
</div>

Якщо після вставки коду ви бачите повідомлення "this is a demo message":

- Переконайтеся, що ви увійшли у свій обліковий запис fastcomments.com.
- Переконайтеся, що у вас увімкнені сторонні файли cookie.
- Потім оновіть цю сторінку та скопіюйте фрагмент коду ще раз. У ньому має бути заповнений `tenantId` ідентифікатором вашого тенанта.