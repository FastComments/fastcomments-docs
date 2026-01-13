U narednom koraku treba da kopirate unapred pripremljeni kod widgeta ispod.

Dok ste prijavljeni na FastComments.com, donji kod će već sadržavati informacije o vašem nalogu. Kopirajmo ga:

[inline-code-attrs-start title = 'Kôd za Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Očisti postojeću instancu
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Ukloni postojeću gornju traku ako postoji
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Kreiraj novu gornju traku
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Inicijalizuj FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Ažuriraj trenutni pathname
            currentPathname = window.location.pathname;
        }

        // Početno učitavanje
        load();

        // Proveravaj na promene svakih 500ms
        setInterval(() => {
            // Ponovo učitaj ako se pathname promenio
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ponovo učitaj ako je widget uklonjen
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ponovo učitaj ako je kontejner ispraznjen
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">Pasted Code</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Pasted Code" />
</div>

Ako vidite "ovo je demo poruka" nakon lepljenja koda:

- Proverite da ste prijavljeni na svoj fastcomments.com nalog.
- Proverite da su kolačići trećih strana omogućeni.
- Zatim osvežite ovu stranu i ponovo kopirajte isječak koda. Trebalo bi da `tenantId` bude popunjen identifikatorom vašeg tenanta.