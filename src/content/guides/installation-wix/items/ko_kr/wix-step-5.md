다음으로, 댓글 쓰레드가 현재 페이지에 따라 변경되어 사용자가 현재 표시된 콘텐츠에 대해 토론할 수 있도록 설정하겠습니다.

다음 단계를 수행하지 않으면 사이트 전체에 대해 하나의 전역 댓글 쓰레드만 있게 되며, 이는 별로 유용하지 않습니다.

#### 개발 모드

이 기능을 추가하려면 Wix에서 `Dev Mode`라고 부르는 모드로 들어가야 합니다.

화면 상단에서 `Dev Mode` 옵션을 클릭하세요.

<div class="screenshot white-bg">
    <div class="title">개발 모드 활성화</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="개발 모드 활성화" />
</div>

#### 요소 ID 설정

이를 위해 커스텀 코드를 추가할 예정이지만, 먼저 새 임베드 요소를 참조할 수 있도록 ID를 부여해야 합니다.

이름을 `fastcomments`로 하겠습니다.

추가한 새 임베드 요소를 클릭하면, 개발 모드에서 오른쪽 아래에 `html1`과 같은 값이 있는 ID 필드를 볼 수 있습니다:

<div class="screenshot white-bg">
    <div class="title">ID 필드</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID 필드" />
</div>

이 값을 `fastcomments`로 변경한 뒤 Enter 키를 누르세요:

<div class="screenshot white-bg">
    <div class="title">ID 설정</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="ID 설정" />
</div>

이제 댓글 영역에 현재 우리가 보고 있는 페이지를 알려주는 커스텀 코드를 추가할 수 있습니다.

화면 하단에서 다음과 같은 코드 편집기를 볼 수 있습니다:

<div class="screenshot white-bg">
    <div class="title">편집기 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="편집기 열기" />
</div>

다음 코드를 복사하여 그곳에 붙여넣으세요:

[inline-code-attrs-start title = 'Wix 댓글 네비게이션 스니펫'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">네비게이션 코드 추가</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="네비게이션 코드 추가" />
</div>

---