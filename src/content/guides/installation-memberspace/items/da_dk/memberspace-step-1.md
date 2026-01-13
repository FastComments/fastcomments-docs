Vi kan nemt forbinde FastComments med MemberSpace med et lille kodestykke:

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

Når brugeren besøger dit websted eller din applikation, mens de er logget ind via MemberStack, vil de automatisk blive logget ind på FastComments, og deres kommentarer
vil blive markeret som `Verified`.

Derudover, i ovenstående eksempel, hvis du har en abonnementsplan kaldet `VIP Plan`, vil vi vise et `VIP`-badge ved siden af brugerens navn. Du kan redigere eksemplet for at
tilføje flere planer. Kontakt support, hvis du har spørgsmål.

### Tillad anonym kommentering

Hvis du også gerne vil have **anonym kommentering**, skal du sætte ALLOW_ANON til true sådan her:

    const ALLOW_ANON = true;

Husk også at ændre `https://example.com/login` til det sted, hvor du vil have brugerne til at gå hen, når de klikker på `Login`-knappen:

På denne måde vil brugerne have mulighed for at indtaste deres navn og e-mail for at kommentere, hvis de ikke er logget ind på dit medlemswebsted.
