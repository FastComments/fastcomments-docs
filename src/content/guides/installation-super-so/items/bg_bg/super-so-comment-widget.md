## Добавяне на уиджет за живи коментари към вашите Super.so Notion статии

В допълнение към Collab Chat, можете да добавите традиционен уиджет за коментари в долната част на вашите Notion статии. Това позволява на читателите да оставят коментари и да водят дискусии за цялата статия.

### Стъпки за инсталиране

Копирайте следния код и го поставете в секцията `Body` на настройките на вашия сайт в Super.so:

[inline-code-attrs-start title = 'Super.so FastComments Уиджет за живи коментари'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Премахване на съществуващия екземпляр
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Създаване на нов целеви елемент
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Инициализиране на FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
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

            // Презареждане ако уиджетът е премахнат
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Презареждане ако контейнерът е изчистен
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Важни бележки

- Уиджетът за коментари ще се появи в края на вашите Notion статии
- Всяка страница получава своя уникална нишка за коментари, базирана на пътя в URL-а
- Уверете се, че заменяте `"demo"` с вашия реален tenant ID от акаунта ви във FastComments
- Уиджетът автоматично обработва динамичното зареждане на страници на Super.so

---