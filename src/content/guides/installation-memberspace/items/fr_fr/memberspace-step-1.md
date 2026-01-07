Nous pouvons facilement connecter FastComments avec MemberSpace avec un petit extrait de code:

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

Lorsque l'utilisateur visite votre site ou application tout en étant connecté via MemberStack, il sera automatiquement connecté à FastComments et ses commentaires
seront marqués comme `Verified`.

De plus, dans l'exemple ci-dessus, si vous avez un plan d'abonnement appelé `VIP Plan`, nous afficherons un badge `VIP` à côté du nom de l'utilisateur. Vous pouvez modifier l'exemple pour
ajouter plus de plans. Contactez le support si vous avez des questions.

### Autoriser les commentaires anonymes

Si vous souhaitez également avoir des **commentaires anonymes**, définissez ALLOW_ANON sur true comme ceci:

    const ALLOW_ANON = true;

N'oubliez pas également de changer `https://example.com/login` vers l'endroit où vous voulez que les utilisateurs aillent lorsqu'ils cliquent sur le bouton `Login`:

De cette façon, les utilisateurs auront la possibilité d'entrer leur nom et leur e-mail pour commenter s'ils ne sont pas connectés à votre site membre.
