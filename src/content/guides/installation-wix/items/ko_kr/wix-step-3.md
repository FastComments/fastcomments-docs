이 예제는 Wix와 호환되도록 커스텀 코드를 사용합니다. **다른 튜토리얼의 FastComments 코드 스니펫은 사용할 수 없습니다.**

`Enter Code`를 클릭하고 `HTML`을 선택하여 커스텀 HTML 대화상자를 추가하는 양식을 엽니다:

<div class="screenshot white-bg">
    <div class="title">3단계: HTML 대화상자 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="3단계: HTML 대화상자 열기" />
</div>

다음 HTML 스니펫을 복사하여 대화상자에 붙여넣고, Update를 클릭합니다:

[inline-code-attrs-start title = 'Wix 댓글 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">3단계: 붙여넣기 및 저장</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="3단계: 붙여넣기 및 저장" />
</div>

이제 매우 작은 크기의 댓글 위젯이 표시됩니다:

<div class="screenshot white-bg">
    <div class="title">3단계: 결과</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="3단계: 결과" />
</div>

다음으로 이것을 페이지에 맞게 위치와 크기를 조정하겠습니다.