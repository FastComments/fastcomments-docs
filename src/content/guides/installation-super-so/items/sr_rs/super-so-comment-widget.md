## Додавање лайв видгета за коментаре на ваше Super.so Notion чланке

Поред Collab Chat-а, можете додати традиционални видгет за коментаре на дно ваших Notion чланака. Ово омогућава читаоцима да остављају коментаре и воде дискусије о целом чланку.

### Кораци инсталације

Копирајте следећи код и налепите га у одељак `Body` у подешавањима ваше Super.so странице:

[inline-code-attrs-start title = 'Super.so FastComments видгет за коментаре уживо'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Очисти постојећу инстанцу
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Креирај нови циљ
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Иницијализуј FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Ажурирај тренутну путању
            currentPathname = window.location.pathname;
        }

        // Почетно учитавање
        load();

        // Проверавај на сваких 500ms за промене
        setInterval(() => {
            // Поново учитај ако се путања променила
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Поново учитај ако је видгет уклоњен
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Поново учитај ако је контејнер испражњен
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Важне напомене

- Видгет за коментаре ће се појавити на дну ваших Notion чланака
- Свака страница добија своју јединствену нит коментара на основу путање URL-а
- Обавезно замените `"demo"` вашим стварним tenant ID-јем из вашег FastComments налога
- Видгет аутоматски рукује Super.so динамичким учитавањем страница