Можем лесно да свържем FastComments с MemberSpace с малък код:

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

Когато потребителят посети вашия сайт или приложение, докато е влязъл чрез MemberStack, той автоматично ще бъде влязъл във FastComments и коментарите му
ще бъдат маркирани като `Verified`.

Освен това, в горния пример, ако имате абонаментен план, наречен `VIP Plan`, ще покажем значка `VIP` до името на потребителя. Можете да редактирате примера, за да
добавите повече планове. Свържете се с поддръжката, ако имате въпроси.

### Разрешаване на анонимно коментиране

Ако искате да имате и **анонимно коментиране**, задайте ALLOW_ANON на true по следния начин:

    const ALLOW_ANON = true;

Също така не забравяйте да промените `https://example.com/login` към мястото, където искате потребителите да отидат, когато щракнат върху бутона `Login`:

По този начин потребителите ще имат възможност да въведат своето име и имейл, за да коментират, ако не са влезли в членския ви сайт.
