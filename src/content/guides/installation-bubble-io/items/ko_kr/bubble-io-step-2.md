방금 추가한 HTML 요소를 클릭하세요. 표시되는 속성 편집기에서 HTML 필드에 다음 코드를 붙여넣으세요:

[inline-code-attrs-start title = 'Bubble.io 라이브 댓글 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble은 코드 스니펫을 비동기(async)로 변경하는 경향이 있습니다
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

<div class="screenshot white-bg">
    <div class="title">FastComments 코드 삽입</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="HTML 요소에 FastComments 코드 삽입" />
</div>

참고: 이 코드는 Bubble의 동적 환경에서 FastComments가 올바르게 로드되도록 재시도 메커니즘을 포함합니다.
다른 코드 스니펫은 작동하지 않습니다.

가입 후 `'demo'`를 실제 FastComments 테넌트 ID로 바꿔야 합니다. FastComments.com에 로그인되어 있으면 이미 바뀌어 있을 것입니다.