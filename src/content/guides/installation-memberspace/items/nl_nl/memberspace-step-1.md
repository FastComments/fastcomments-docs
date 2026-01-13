We kunnen FastComments eenvoudig verbinden met MemberSpace met een klein codefragment:

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

Wanneer de gebruiker uw site of applicatie bezoekt terwijl hij is ingelogd via MemberStack, wordt hij automatisch ingelogd bij FastComments en worden zijn reacties
gemarkeerd als `Verified`.

Bovendien, als u in het bovenstaande voorbeeld een abonnementsplan heeft met de naam `VIP Plan`, tonen we een `VIP`-badge naast de naam van de gebruiker. U kunt het voorbeeld bewerken om
meer plannen toe te voegen. Neem contact op met ondersteuning als u vragen heeft.

### Anoniem reageren toestaan

Als u ook **anoniem reageren** wilt toestaan, stel ALLOW_ANON in op true als volgt:

    const ALLOW_ANON = true;

Vergeet ook niet om `https://example.com/login` te wijzigen naar waar u wilt dat gebruikers naartoe gaan wanneer ze op de `Login`-knop klikken:

Op deze manier hebben gebruikers de mogelijkheid om hun naam en e-mail in te voeren om te reageren als ze niet zijn ingelogd op uw ledensite.
