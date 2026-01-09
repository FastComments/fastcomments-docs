V naslednjem koraku morate kopirati spodnjo vnaprej pripravljeno kodo widgeta.

Dokler ste prijavljeni v FastComments.com bo spodnja koda že vsebovala podatke o vašem računu. Kopirajmo jo:

[inline-code-attrs-start title = 'Koda Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Počisti obstoječo instanco
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Počisti obstoječo zgornjo vrstico, če obstaja
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Ustvari novo zgornjo vrstico
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Inicializiraj FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Posodobi currentPathname
            currentPathname = window.location.pathname;
        }

        // Začetno nalaganje
        load();

        // Preverjaj spremembe vsakih 500ms
        setInterval(() => {
            // Znova naloži, če se je pathname spremenil
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Znova naloži, če je bil widget odstranjen
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Znova naloži, če je bil vsebnik izpraznjen
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Zdaj prilepite v polje `Body`:

<div class="screenshot white-bg">
    <div class="title">Prilepljena koda</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Prilepljena koda" />
</div>

Če po prilepitvi kode vidite sporočilo "this is a demo message":

- Prepričajte se, da ste prijavljeni v svoj račun na fastcomments.com.
- Prepričajte se, da imate omogočene piškotke tretjih oseb.
- Nato osvežite to stran in znova kopirajte odsek s kodo. V njem bi moral biti `tenantId` izpolnjen z identifikatorjem vašega najemnika.