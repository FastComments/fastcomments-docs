Ми можемо легко підключити FastComments до MemberSpace за допомогою невеликого фрагмента коду:

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

Коли користувач відвідує ваш сайт або додаток, будучи авторизованим через MemberStack, він автоматично увійде в FastComments, і його коментарі
будуть позначені як `Verified`.

Крім того, у наведеному вище прикладі, якщо у вас є план підписки під назвою `VIP Plan`, ми відобразимо значок `VIP` поруч з іменем користувача. Ви можете відредагувати приклад, щоб
додати більше планів. Зверніться до служби підтримки, якщо у вас є питання.

### Дозволити анонімне коментування

Якщо ви також хочете мати **анонімне коментування**, встановіть ALLOW_ANON на true наступним чином:

    const ALLOW_ANON = true;

Також не забудьте змінити `https://example.com/login` на адресу, куди ви хочете направити користувачів при натисканні кнопки `Login`:

Таким чином, користувачі зможуть ввести своє ім'я та email для коментування, якщо вони не увійшли на ваш членський сайт.
