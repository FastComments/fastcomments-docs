---
В следващата стъпка трябва да копирате готовия код на джаджа по-долу.

Докато сте влезли в профила си в FastComments.com, следният кодов откъс вече ще съдържа информацията за вашия акаунт. Нека го копираме:

[inline-code-attrs-start title = 'Код за Collab Chat на Super.so FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Почистване на съществуващия екземпляр
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Почистване на съществуващата горна лента, ако съществува
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Създаване на нова горна лента
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Инициализация на FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Актуализиране на текущия pathname
            currentPathname = window.location.pathname;
        }

        // Първоначално зареждане
        load();

        // Проверка за промени на всеки 500 ms
        setInterval(() => {
            // Презареждане ако pathname се е променил
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Презареждане ако джаджата е премахната
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Презареждане ако контейнерът е изпразнен
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Сега го поставете в областта `Body`:

<div class="screenshot white-bg">
    <div class="title">Поставен код</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Поставен код" />
</div>

If you see a "this is a demo message" after pasting the code:

- Уверете се, че сте влязли във вашия акаунт в fastcomments.com.
- Уверете се, че имате разрешени бисквитки на трети страни.
- След това презаредете тази страница и копирайте кода отново. Той трябва да има попълнен `tenantId` с идентификатора на вашия tenant.

---