FastComments lahko enostavno povežemo z MemberSpace z majhnim delčkom kode:

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

Ko uporabnik obišče vašo spletno stran ali aplikacijo, medtem ko je prijavljen prek MemberStack, bo samodejno prijavljen v FastComments in njegovi komentarji
bodo označeni kot `Verified`.

Poleg tega bomo v zgornjem primeru, če imate naročniški načrt z imenom `VIP Plan`, prikazali značko `VIP` poleg uporabniškega imena. Primer lahko uredite, da
dodate več načrtov. Obrnite se na podporo, če imate vprašanja.

### Dovoli anonimno komentiranje

Če želite imeti tudi **anonimno komentiranje**, nastavite ALLOW_ANON na true takole:

    const ALLOW_ANON = true;

Prav tako ne pozabite spremeniti `https://example.com/login` na mesto, kamor želite, da uporabniki gredo, ko kliknejo gumb `Login`:

Na ta način bodo uporabniki imeli možnost vnesti svoje ime in e-pošto za komentiranje, če niso prijavljeni na vašo člansko stran.
