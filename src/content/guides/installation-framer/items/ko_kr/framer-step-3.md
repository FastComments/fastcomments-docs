아래는 Framer용 Live Comments FastComments 스니펫입니다.

[inline-code-attrs-start title = 'FastComments Framer 전용 댓글 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 일부 제공업체는 코드 스니펫을 비동기(async)로 변경합니다
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

또는, 대안으로 스트리밍 채팅 위젯을 사용할 수 있습니다. Framer용 Streaming Chat FastComments 스니펫은 다음과 같습니다:

[inline-code-attrs-start title = 'FastComments Framer 전용 스트리밍 채팅 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // 일부 제공업체는 코드 스니펫을 비동기(async)로 변경합니다
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

FastComments는 Framer 에디터를 지원하므로 코드를 붙여넣으면 다음과 같은 화면을 볼 수 있습니다( `Publish`를 클릭해야 할 수 있습니다):

<div class="screenshot white-bg">
    <div class="title">댓글 위젯 미리보기</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="댓글 위젯 미리보기" />
</div>

이제 사이트를 열면 댓글 영역이 표시됩니다! 원하는 경우 Framer의 사이드바에서 위젯을 전체 너비로 설정할 수도 있습니다.

참고로 Framer는 위젯의 높이를 제한하고 자동 크기 조정을 지원하지 않으므로 여기서는 고정 높이인 Live Chat
위젯을 선택했습니다.