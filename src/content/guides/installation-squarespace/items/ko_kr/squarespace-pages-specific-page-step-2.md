이제 다음 코드 스니펫을 복사할 수 있습니다. 스니펫 오른쪽 상단에 나타나는 복사 버튼을 사용하세요.

코드에서 몇 가지 설정을 변경할 수 있습니다. 4행에서 7행을 확인하세요.

[inline-code-attrs-start title = 'Squarespace 단일 페이지 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 귀하의 계정 ID

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

다음과 같이 보일 것입니다:

<div class="screenshot white-bg">
    <div class="title">붙여넣기 및 저장</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="붙여넣기 및 저장" />
</div>

이제 오른쪽 상단의 저장을 클릭하세요.

참고: `Preview in Safe Mode` 옵션은 작동하지 않지만, 사이트를 방문하면 위젯이 표시됩니다.

문제가 발생하면, 하단 근처에 `"tenantId": "demo"`라고 표시되지 않았는지 확인하세요. 로그인되어 있으면 해당 위치에 귀하의 tenant id가 표시되어야 합니다. 표시되지 않으면 지원팀에 문의하세요.