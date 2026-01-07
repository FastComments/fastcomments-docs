Možemo lako povezati FastComments s MemberSpace pomoću malog isječka koda:

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

Kada korisnik posjeti vašu stranicu ili aplikaciju dok je prijavljen putem MemberStack-a, automatski će biti prijavljen u FastComments i njihovi komentari
bit će označeni kao `Verified`.

Dodatno, u gornjem primjeru, ako imate pretplatnički plan nazvan `VIP Plan`, prikazat ćemo `VIP` značku pored korisničkog imena. Možete urediti primjer da
dodate više planova. Obratite se podršci ako imate pitanja.

### Dopusti anonimno komentiranje

Ako želite imati i **anonimno komentiranje**, postavite ALLOW_ANON na true ovako:

    const ALLOW_ANON = true;

Također ne zaboravite promijeniti `https://example.com/login` na mjesto gdje želite da korisnici odu kada kliknu gumb `Login`:

Na taj način korisnici će imati mogućnost unijeti svoje ime i e-mail za komentiranje ako nisu prijavljeni na vašu člansku stranicu.
