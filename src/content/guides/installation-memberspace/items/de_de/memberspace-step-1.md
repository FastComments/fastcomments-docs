Wir können FastComments ganz einfach mit MemberSpace über einen kleinen Code-Schnipsel verbinden:

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

Wenn der Benutzer Ihre Website oder Anwendung besucht, während er über MemberStack angemeldet ist, wird er automatisch bei FastComments angemeldet und seine Kommentare
werden als `Verified` markiert.

Zusätzlich wird in dem obigen Beispiel, wenn Sie einen Abonnementplan namens `VIP Plan` haben, ein `VIP`-Abzeichen neben dem Benutzernamen angezeigt. Sie können das Beispiel bearbeiten, um
weitere Pläne hinzuzufügen. Wenden Sie sich an den Support, wenn Sie Fragen haben.

### Anonymes Kommentieren erlauben

Wenn Sie auch **anonymes Kommentieren** ermöglichen möchten, setzen Sie ALLOW_ANON wie folgt auf true:

    const ALLOW_ANON = true;

Denken Sie auch daran, `https://example.com/login` auf die Stelle zu ändern, zu der Benutzer gelangen sollen, wenn sie auf die `Login`-Schaltfläche klicken:

Auf diese Weise haben Benutzer die Möglichkeit, ihren Namen und ihre E-Mail-Adresse einzugeben, um zu kommentieren, wenn sie nicht auf Ihrer Mitgliederseite angemeldet sind.
