Possiamo collegare facilmente FastComments con MemberSpace con un piccolo frammento di codice:

[inline-code-attrs-start title = 'FastComments MemberSpace Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo';
        const ALLOW_ANON = false;
        const LOGIN_URL = 'https://example.com/login';
        const PLAN_DISPLAY_LABELS = {
            'VIP Plan': 'VIP'
        };
        let lastInstance;

        function tick() {
            if (!window.MemberSpace) {
                return setTimeout(tick, 100);
            }
            if (!window.FastCommentsUI) {
                return setTimeout(tick, 100);
            }
            const target = document.getElementById('fastcomments-widget');
            if (!target) {
                return setTimeout(tick, 100);
            }
            const data = MemberSpace.getMemberInfo();
            if (data.isLoggedIn && data.memberInfo) {
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: {
                        displayLabel: getDisplayLabel(data.memberInfo),
                        username: data.memberInfo.name,
                        email: data.memberInfo.email,
                        avatar: data.memberInfo.profileImageUrl
                    }
                });
            } else if (lastInstance) {
                lastInstance.destroy();
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: getAnonSSOConfig()
                });
            }
        }

        function getAnonSSOConfig() {
            if (ALLOW_ANON) {
                return undefined;
            }
            return {
                loginURL: LOGIN_URL
            };
        }

        function getDisplayLabel(memberInfo) {
            if (!memberInfo.memberships) {
                return;
            }
            for (const membership of memberInfo.memberships) {
                const label = PLAN_DISPLAY_LABELS[membership.name];
                if (label) {
                    return label
                }
            }
        }

        tick();
    })();
</script>
[inline-code-end]

Quando l'utente visita il tuo sito o applicazione mentre è connesso tramite MemberStack, verrà automaticamente connesso a FastComments e i suoi commenti
saranno contrassegnati come `Verified`.

Inoltre, nell'esempio sopra, se hai un piano di abbonamento chiamato `VIP Plan`, mostreremo un badge `VIP` accanto al nome dell'utente. Puoi modificare l'esempio per
aggiungere più piani. Contatta il supporto se hai domande.

### Consenti commenti anonimi

Se desideri anche avere **commenti anonimi**, imposta ALLOW_ANON su true in questo modo:

    const ALLOW_ANON = true;

Ricorda anche di cambiare `https://example.com/login` con il posto dove vuoi che gli utenti vadano quando cliccano sul pulsante `Login`:

In questo modo gli utenti avranno la possibilità di inserire il loro nome e la loro email per commentare se non sono connessi al tuo sito membri.
