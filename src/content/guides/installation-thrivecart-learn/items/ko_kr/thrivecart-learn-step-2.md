Step 2에서는 코드 스니펫을 복사해야 합니다. 50번째 줄이 "demo"로 되어 있지 않은지 확인하세요 - 여기에는 귀하의 tenant id가 있어야 합니다. 보통 자동으로 채워져 있습니다.

이제 ThriveCart-Learn 전용 FastComments 코드 스니펫을 복사하겠습니다.

ThriveCart와의 통합에는 많은 기능이 포함되어 있어 코드가 꽤 큽니다. 코드 스니펫의 오른쪽 상단에 있는 복사 버튼을 클릭하세요:

[inline-code-attrs-start title = 'ThriveCart Learn+ 댓글 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {optedInNotifications: true, optedInSubscriptionNotifications: true};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // 미리보기에서는 클래스가 다름.
            }
            // ThriveCart가 id를 변경할 경우에 대비한 광범위한 이메일 입력 필드 선택자.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // 미리보기가 작동하도록 허용 - 이메일 없음.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 인터넷 속도가 느릴 경우를 대비해 5회 시도 이후 대기 시간을 늘립니다.
            }
            if (profileLink) {
                // ThriveCart가 이미지 클래스 선택자를 변경할 경우를 대비해 원시 "img" 쿼리를 사용합니다.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // ThriveCart가 프로필 이름 표시 방식을 변경할 경우에 대비해 innerText를 사용합니다.
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                } else {
                    const bold = profileLink.querySelector('b');
                    if (bold && bold.innerText) {
                        isAuthenticated = true;
                        simpleSSO.username = bold.innerText;
                    }
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 인터넷 속도가 느릴 경우를 대비해 5회 시도 이후 대기 시간을 늘립니다.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // 경우에 따라 TC가 동일 페이지에 여러 링크를 사용할 수 있으므로 중복을 제거합니다.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // 마케팅 매개변수와 도메인 이름을 제거합니다.
                url = window.location.pathname;
            }

            if (url) {
                url = url.replace('/starte-hier', '');
                url = url.replace('/start-here', '');
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: url,
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

        function getPathnameFromUrl(url) {
            try {
                const parsedUrl = new URL(url);
                // 마케팅 매개변수와 도메인 이름을 제거합니다.
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // 기본값으로 현재 경로를 사용하므로 적어도 때때로 동작합니다
            }
        }

    })();
</script>
[inline-code-end]

이제 이를 ThriveCart 편집기의 왼쪽 코드 블록에 붙여넣으세요. 다음과 같이 보일 것입니다:

<div class="screenshot white-bg">
    <div class="title">코드 추가됨</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="코드 추가됨" />
</div>

이제 끝났습니다! 이제 게시하기만 하면 됩니다:

<div class="screenshot white-bg">
    <div class="title">게시</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="게시" />
</div>

이제 완료되었습니다! 미리보기할 때 강좌에서 댓글 상자를 볼 수 있으며, 실제 사용자는 **다시 로그인하거나 사용자명/이메일을 두 번째로 입력하지 않고도** 댓글을 남길 수 있습니다.

### 테스트 참고!

익명 댓글 기능이 비활성화되어 있다면(기본값은 비활성화) `Preview` 모드에서 `John Smith` 사용자로 댓글을 남길 수 없습니다. 기본 `John Smith` 사용자에는 이메일이 없어 인증 오류가 발생합니다. 테스트를 원하시면 쿠폰 코드를 사용하여 실제 사용자처럼 사이트를 진행해 보시길 권장합니다.

---