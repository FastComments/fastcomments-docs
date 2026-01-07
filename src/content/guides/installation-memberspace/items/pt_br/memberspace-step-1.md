Podemos conectar facilmente o FastComments com o MemberSpace com um pequeno trecho de código:

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

Quando o usuário visita seu site ou aplicativo enquanto está logado via MemberStack, ele será automaticamente conectado ao FastComments e seus comentários
serão marcados como `Verified`.

Além disso, no exemplo acima, se você tiver um plano de assinatura chamado `VIP Plan`, exibiremos um emblema `VIP` ao lado do nome do usuário. Você pode editar o exemplo para
adicionar mais planos. Entre em contato com o suporte se tiver dúvidas.

### Permitir comentários anônimos

Se você também quiser ter **comentários anônimos**, defina ALLOW_ANON como true assim:

    const ALLOW_ANON = true;

Lembre-se também de alterar `https://example.com/login` para onde você deseja que os usuários vão quando clicarem no botão `Login`:

Dessa forma, os usuários terão a opção de inserir seu nome e e-mail para comentar se não estiverem logados em seu site de membros.
