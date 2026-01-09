У следећем кораку треба да копирате унапред направљени код видгета испод.

Док год сте пријављени на FastComments.com, испод наведени исечак кода ће већ садржати информације о вашем налогу. Хајде да га копирамо:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat Код'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Очисти постојећу инстанцу
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Очисти постојећу горњу траку ако постоји
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Направи нову горњу траку
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Иницијализуј FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Ажурирај тренутну путању
            currentPathname = window.location.pathname;
        }

        // Почетно учитавање
        load();

        // Проверавај сваких 500ms за промене
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
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Сада налепите у област `Body`:

<div class="screenshot white-bg">
    <div class="title">Налепљен код</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Налепљен код" />
</div>

Ако видите „ово је демонстрациона порука“ након лепљења кода:

- Уверите се да сте пријављени на ваш fastcomments.com налог.
- Уверите се да су колачићи треће стране омогућени.
- Затим освежите ову страницу и поново копирајте исечак кода. Он би требао имати `tenantId` попуњен идентификатором вашег tenant-а.

---