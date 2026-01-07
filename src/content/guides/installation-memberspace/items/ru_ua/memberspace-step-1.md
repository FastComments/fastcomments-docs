Мы можем легко подключить FastComments к MemberSpace с помощью небольшого фрагмента кода:

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

Когда пользователь посещает ваш сайт или приложение, будучи авторизованным через MemberStack, он автоматически войдёт в FastComments, и его комментарии
будут отмечены как `Verified`.

Кроме того, в приведённом выше примере, если у вас есть план подписки под названием `VIP Plan`, мы отобразим значок `VIP` рядом с именем пользователя. Вы можете отредактировать пример, чтобы
добавить больше планов. Обратитесь в службу поддержки, если у вас есть вопросы.

### Разрешить анонимное комментирование

Если вы также хотите иметь **анонимное комментирование**, установите ALLOW_ANON в true следующим образом:

    const ALLOW_ANON = true;

Также не забудьте изменить `https://example.com/login` на адрес, куда вы хотите направить пользователей при нажатии кнопки `Login`:

Таким образом, пользователи смогут ввести своё имя и email для комментирования, если они не вошли на ваш членский сайт.
