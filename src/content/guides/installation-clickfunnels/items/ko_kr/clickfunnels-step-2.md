이제 템플릿 편집기에 있으므로 댓글 또는 라이브 채팅을 어디에 표시할지 결정해야 합니다.

이 예제에서는 비디오 바로 아래에 추가합니다. 위젯을 추가할 요소에 마우스를 올리고 `ADD ELEMENT`를 클릭하세요:

<div class="screenshot white-bg">
    <div class="title">요소 추가</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="요소 추가" />
</div>

`CUSTOM JS/HTML`를 선택하세요:

<div class="screenshot white-bg">
    <div class="title">CUSTOM JS/HTML 선택</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="CUSTOM JS/HTML 선택" />
</div>

이제 코드를 붙여넣을 코드 편집기를 열겠습니다.

다음 단계에서 ClickFunnels가 약간 혼란스러울 수 있습니다.

새 요소에 마우스를 올릴 때 `Code`를 선택하지 않는 것이 중요합니다. 대신 `SETTINGS`를 선택하세요:

<div class="screenshot white-bg">
    <div class="title">SETTINGS 선택</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="SETTINGS 선택" />
</div>

이제 오른쪽에서 `Open Code Editor`를 클릭할 수 있습니다:

<div class="screenshot white-bg">
    <div class="title">Open Code Editor 클릭</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Open Code Editor 클릭" />
</div>

큰 사각형이 열리는 것을 보게 됩니다. 여기에 코드를 붙여넣습니다. 다음 스니펫을 복사하세요(오른쪽 상단의 복사 버튼을 사용):

[inline-code-attrs-start title = 'ClickFunnels 스트리밍 채팅 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 일부 제공자는 코드 스니펫을 비동기(async)로 변경합니다
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

이 코드 스니펫은 비디오와 잘 어울리는 Streaming Chat 제품용입니다. 대신 일반 페이지나 블로그 게시물에 가장 적합한 Live Commenting 위젯 코드 스니펫을 원하면 이 튜토리얼의 끝에 있습니다.

코드 스니펫을 창에 붙여넣으면 다음과 같이 보입니다:

<div class="screenshot white-bg">
    <div class="title">코드 붙여넣기</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="코드 붙여넣기" />
</div>

이제 창을 닫기만 하면 됩니다:

<div class="screenshot white-bg">
    <div class="title">닫기</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="닫기" />
</div>

이제 변경 사항을 미리보기할 수 있습니다! 위젯을 옮겨 보면서 가장 적합한 위치를 찾아보세요.

<div class="screenshot white-bg">
    <div class="title">미리보기</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="미리보기" />
</div>

성공했습니다! 모바일 테스트를 잊지 마세요!

<div class="screenshot white-bg">
    <div class="title">성공!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="성공!" />
</div>