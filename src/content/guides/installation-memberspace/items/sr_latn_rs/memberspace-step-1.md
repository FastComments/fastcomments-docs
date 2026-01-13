Možemo lako povezati FastComments sa MemberSpace pomoću malog isečka koda:

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

Kada korisnik poseti vašu stranicu ili aplikaciju dok je prijavljen preko MemberStack-a, automatski će biti prijavljen u FastComments i njihovi komentari
će biti označeni kao `Verified`.

Pored toga, u gornjem primeru, ako imate plan pretplate pod nazivom `VIP Plan`, prikazaćemo `VIP` značku pored korisničkog imena. Možete urediti primer da
dodate više planova. Obratite se podršci ako imate pitanja.

### Dozvoli anonimno komentarisanje

Ako želite da imate i **anonimno komentarisanje**, postavite ALLOW_ANON na true ovako:

    const ALLOW_ANON = true;

Takođe ne zaboravite da promenite `https://example.com/login` na mesto gde želite da korisnici odu kada kliknu na dugme `Login`:

Na ovaj način korisnici će imati mogućnost da unesu svoje ime i e-poštu za komentarisanje ako nisu prijavljeni na vašu člansku stranicu.
