Weebly와 FastComments 통합을 제대로 작동시키려면 **두 개의** 작은 코드 조각을 추가해야 합니다.

첫 번째 스니펫은 Weebly의 "Comments are Closed" 메시지를 숨기기 위한 것이고, 두 번째는 실제로 FastComments를 불러오는 것입니다.

먼저, 이 작은 코드 스니펫을 복사하세요:

[inline-code-attrs-start title = 'FastComments 헤더 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

그런 다음, `Step One`에서 사용한 동일한 설정 페이지에서 `Post header code` 옆의 `+`를 클릭하세요.

<div class="screenshot white-bg">
    <div class="title">게시물 헤더 코드 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="게시물 헤더 코드 열기" />
</div>

다음과 같이 텍스트 박스가 열립니다:

<div class="screenshot white-bg">
    <div class="title">게시물 헤더 코드 열림</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="게시물 헤더 코드 열림" />
</div>

이제 우리의 코드 스니펫을 붙여넣습니다:

<div class="screenshot white-bg">
    <div class="title">헤더 코드 스니펫 붙여넣음</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="헤더 코드 스니펫 붙여넣음" />
</div>

다음으로 FastComments를 활성화할 푸터 코드를 추가합니다. `Post footer code` 옆의 더하기 기호를 클릭하세요:

<div class="screenshot white-bg">
    <div class="title">게시물 푸터 코드 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="게시물 푸터 코드 열기" />
</div>

Weebly를 위해 **특히 설계된** 이 코드 스니펫을 복사하세요:

[inline-code-attrs-start title = 'FastComments 푸터 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // remove show comments button
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

이제 푸터 코드를 붙여넣습니다:

<div class="screenshot white-bg">
    <div class="title">게시물 푸터 코드 추가됨</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="게시물 푸터 코드 추가됨" />
</div>

그게 전부입니다!

---