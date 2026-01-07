Можемо лако повезати FastComments са MemberSpace помоћу малог исечка кода:

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

Када корисник посети вашу страницу или апликацију док је пријављен преко MemberStack-а, аутоматски ће бити пријављен у FastComments и њихови коментари
ће бити означени као `Verified`.

Поред тога, у горњем примеру, ако имате план претплате под називом `VIP Plan`, приказаћемо `VIP` значку поред корисничког имена. Можете уредити пример да
додате више планова. Обратите се подршци ако имате питања.

### Дозволи анонимно коментарисање

Ако желите да имате и **анонимно коментарисање**, поставите ALLOW_ANON на true овако:

    const ALLOW_ANON = true;

Такође не заборавите да промените `https://example.com/login` на место где желите да корисници оду када кликну на дугме `Login`:

На овај начин корисници ће имати могућност да унесу своје име и е-пошту за коментарисање ако нису пријављени на вашу чланску страницу.
