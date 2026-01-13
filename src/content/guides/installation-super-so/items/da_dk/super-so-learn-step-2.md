---
I dette næste trin skal du kopiere den forudlavede widget-kode nedenfor.

Så længe du er logget ind på FastComments.com, vil kodeudsnittet nedenfor allerede indeholde dine kontooplysninger. Lad os kopiere det:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat-kode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Ryd op i eksisterende instans
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Ryd op i eksisterende topbar, hvis den findes
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Opret ny topbar
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Initialiser FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Opdater nuværende sti
            currentPathname = window.location.pathname;
        }

        // Initial indlæsning
        load();

        // Tjek for ændringer hvert 500 ms
        setInterval(() => {
            // Genindlæs hvis sti ændret
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Genindlæs hvis widget blev fjernet
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Genindlæs hvis container blev tømt
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Nu indsæt i `Body`-området:

<div class="screenshot white-bg">
    <div class="title">Indsat kode</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Indsat kode" />
</div>

Hvis du ser en "dette er en demo-meddelelse" efter indsættelse af koden:

- Sørg for, at du er logget ind på din fastcomments.com-konto.
- Sørg for, at tredjeparts-cookies er aktiveret.
- Opdater derefter denne side og kopier kodeudsnittet igen. Det burde have `tenantId` udfyldt med din tenant-identifikator.

---