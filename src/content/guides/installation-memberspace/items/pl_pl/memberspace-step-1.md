Możemy łatwo połączyć FastComments z MemberSpace za pomocą małego fragmentu kodu:

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

Gdy użytkownik odwiedzi Twoją stronę lub aplikację będąc zalogowanym przez MemberStack, zostanie automatycznie zalogowany do FastComments, a jego komentarze
będą oznaczone jako `Verified`.

Ponadto, w powyższym przykładzie, jeśli masz plan subskrypcji o nazwie `VIP Plan`, wyświetlimy odznakę `VIP` obok nazwy użytkownika. Możesz edytować przykład, aby
dodać więcej planów. Skontaktuj się z pomocą techniczną, jeśli masz pytania.

### Zezwól na anonimowe komentowanie

Jeśli chcesz również mieć **anonimowe komentowanie**, ustaw ALLOW_ANON na true w następujący sposób:

    const ALLOW_ANON = true;

Pamiętaj również, aby zmienić `https://example.com/login` na miejsce, do którego chcesz przekierować użytkowników po kliknięciu przycisku `Login`:

W ten sposób użytkownicy będą mieli możliwość wpisania swojego imienia i adresu e-mail, aby komentować, jeśli nie są zalogowani na Twojej stronie członkowskiej.
