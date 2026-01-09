이제 다음 코드 스니펫을 복사할 수 있습니다. 코드 스니펫 우측 상단에 나타나는 복사 버튼을 사용하세요.

코드에서 구성할 수 있는 항목이 몇 가지 있으니, 4행부터 7행을 확인하세요.

[inline-code-attrs-start title = 'Squarespace 모든 페이지 댓글 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 계정 ID

        function tryLoad() {
            // 다양한 레이아웃에 대해 로드 시도
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...그런 다음 코드 영역에 붙여넣고 저장을 클릭하세요. `FOOTER` 블록에 코드가 들어있으면 아래와 같이 표시됩니다:

<div class="screenshot white-bg">
    <div class="title">붙여넣기 및 저장</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="붙여넣기 및 저장" />
</div>

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.