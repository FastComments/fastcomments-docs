이제 다음 코드 스니펫을 복사할 수 있습니다(스니펫 오른쪽 상단의 복사 버튼을 사용하세요):

[inline-code-attrs-start title = 'Squarespace 블로그 댓글 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // your account id

        function tryLoad() {
            // try to load for different layouts
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

...그런 다음 코드 영역에 붙여넣고 저장을 클릭하세요:

<div class="screenshot white-bg">
    <div class="title">붙여넣기 및 저장</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="붙여넣기 및 저장" />
</div>