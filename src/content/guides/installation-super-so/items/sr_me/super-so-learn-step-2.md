---
У следећем кораку треба да копирате унапред припремљени код видгета у наставку.

Док год сте пријављени на FastComments.com, доњи исечак кода већ ће садржати информације о вашем налогу. Хајде да га копирамо:

[inline-code-attrs-start title = 'Код Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Уклони постојећу инстанцу
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Уклони постојећу горњу траку ако постоји
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

            // Ажурирај текући pathname
            currentPathname = window.location.pathname;
        }

        // Почетно учитавање
        load();

        // Проверавај на сваких 500 ms ради промена
        setInterval(() => {
            // Поново учитај ако је pathname промењен
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
    <div class="title">Налепљени код</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Налепљени код" />
</div>

If you see a "this is a demo message" after pasting the code:

- Уверите се да сте пријављени на свој налог на fastcomments.com.
- Уверите се да су колачићи трећих страна омогућени.
- Затим освежите ову страницу и поново копирајте исечак кода. Требало би да има `tenantId` попуњен идентификатором вашег налога.

---