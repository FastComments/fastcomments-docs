Küçük bir kod parçacığıyla FastComments'i MemberSpace ile kolayca bağlayabiliriz:

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

Kullanıcı MemberStack üzerinden giriş yapmışken sitenizi veya uygulamanızı ziyaret ettiğinde, otomatik olarak FastComments'e giriş yapacak ve yorumları
`Verified` olarak işaretlenecektir.

Ayrıca, yukarıdaki örnekte `VIP Plan` adında bir abonelik planınız varsa, kullanıcının adının yanında `VIP` rozeti görüntüleyeceğiz. Daha fazla plan eklemek için örneği
düzenleyebilirsiniz. Sorularınız varsa destek ile iletişime geçin.

### Anonim yorumlara izin ver

**Anonim yorumlara** da izin vermek istiyorsanız, ALLOW_ANON'u şu şekilde true olarak ayarlayın:

    const ALLOW_ANON = true;

Ayrıca `https://example.com/login` adresini, kullanıcıların `Login` düğmesine tıkladıklarında gitmelerini istediğiniz yere değiştirmeyi unutmayın:

Bu şekilde, üye sitenize giriş yapmamış kullanıcılar yorum yapmak için adlarını ve e-postalarını girme seçeneğine sahip olacaklar.
