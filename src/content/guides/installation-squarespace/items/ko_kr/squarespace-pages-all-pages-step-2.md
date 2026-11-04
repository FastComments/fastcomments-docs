이제 다음 코드 스니펫을 복사할 수 있습니다. 스니펫 오른쪽 상단에 나타나는 복사 버튼을 사용하세요.

코드에서 설정할 수 있는 몇 가지 항목이 있습니다. 4번 줄부터 7번 줄을 참고하세요.

[inline-code-attrs-start title = 'Squarespace 모든 페이지 댓글 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...그런 다음 코드 영역에 붙여넣고 저장을 클릭하세요. 코드가 `FOOTER` 블록에 있는 다음과 같은 모습이 되어야 합니다:

<div class="screenshot white-bg">
    <div class="title">붙여넣기 및 저장</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="붙여넣기 및 저장" />
</div>

문제가 발생하면 하단 근처에 `"tenantId": "demo"`라고 표시되지 않는지 확인하세요. 로그인한 상태라면 귀하의 테넌트 ID가 표시되어야 합니다. 그렇지 않다면 지원팀에 문의하세요.