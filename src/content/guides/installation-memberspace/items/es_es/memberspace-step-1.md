Podemos conectar fácilmente FastComments con MemberSpace con un pequeño fragmento de código:

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

Cuando el usuario visita su sitio o aplicación mientras está conectado a través de MemberStack, iniciará sesión automáticamente en FastComments y sus comentarios
se marcarán como `Verified`.

Además, en el ejemplo anterior, si tiene un plan de suscripción llamado `VIP Plan`, mostraremos una insignia `VIP` junto al nombre del usuario. Puede editar el ejemplo para
agregar más planes. Contacte con soporte si tiene preguntas.

### Permitir comentarios anónimos

Si también desea tener **comentarios anónimos**, establezca ALLOW_ANON en true de esta manera:

    const ALLOW_ANON = true;

También recuerde cambiar `https://example.com/login` a donde desea que los usuarios vayan cuando hagan clic en el botón `Login`:

De esta manera, los usuarios tendrán la opción de ingresar su nombre y correo electrónico para comentar si no han iniciado sesión en su sitio de miembros.
