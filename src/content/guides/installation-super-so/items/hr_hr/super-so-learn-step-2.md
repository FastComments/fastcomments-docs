U sljedećem koraku trebate kopirati unaprijed pripremljeni kod widgeta u nastavku.

Sve dok ste prijavljeni na FastComments.com donji isječak koda već će imati informacije o vašem računu. Kopirajmo ga:

[inline-code-attrs-start title = 'Super.so FastComments Collab Chat Kod'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

            // Inicijaliziraj FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Ažuriraj trenutni pathname
            currentPathname = window.location.pathname;
        }

        // Početno učitavanje
        load();

        // Provjera promjena svakih 500 ms
        setInterval(() => {
            // Ponovno učitaj ako se pathname promijenio
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Ponovno učitaj ako je widget uklonjen
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Ponovno učitaj ako je kontejner ispraznjen
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Sada zalijepite u područje `Body`:

<div class="screenshot white-bg">
    <div class="title">Zalijepljeni kod</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Zalijepljeni kod" />
</div>

Ako nakon lijepljenja koda vidite "this is a demo message":

- Provjerite jeste li prijavljeni na svoj račun na fastcomments.com.
- Provjerite jesu li omogućeni kolačići trećih strana.
- Zatim osvježite ovu stranicu i ponovno kopirajte isječak koda. Trebao bi imati `tenantId` popunjen identifikatorom vašeg tenanta.