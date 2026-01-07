작은 코드 스니펫으로 FastComments와 MemberSpace를 쉽게 연결할 수 있습니다:

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

사용자가 MemberStack을 통해 로그인한 상태에서 귀하의 사이트나 애플리케이션을 방문하면, 자동으로 FastComments에 로그인되고 댓글은
`Verified`로 표시됩니다.

또한 위의 예에서 `VIP Plan`이라는 구독 플랜이 있으면 사용자 이름 옆에 `VIP` 배지를 표시합니다. 예제를 편집하여
더 많은 플랜을 추가할 수 있습니다. 질문이 있으시면 지원팀에 문의하세요.

### 익명 댓글 허용

**익명 댓글**도 허용하려면 ALLOW_ANON을 다음과 같이 true로 설정하세요:

    const ALLOW_ANON = true;

또한 `https://example.com/login`을 사용자가 `Login` 버튼을 클릭할 때 이동할 위치로 변경하는 것을 잊지 마세요:

이렇게 하면 회원 사이트에 로그인하지 않은 사용자도 이름과 이메일을 입력하여 댓글을 달 수 있는 옵션이 제공됩니다.
