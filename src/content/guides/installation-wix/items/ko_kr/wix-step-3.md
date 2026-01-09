This example uses custom code to be compatible with Wix. **다른 튜토리얼의 FastComments 코드 스니펫을 사용할 수 없습니다.**

Open the form to add our custom HTML dialog by clicking `Enter Code` and selecting `HTML`:

<div class="screenshot white-bg">
    <div class="title">3단계: HTML 대화 상자 열기</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="3단계: HTML 대화 상자 열기" />
</div>

Copy the following HTML snippet and paste it into the dialog, and click Update:

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

You should now see a very tiny version of the comment widget:

<div class="screenshot white-bg">
    <div class="title">3단계: 결과</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="3단계: 결과" />
</div>

다음으로 이 요소의 위치와 크기를 페이지에 맞게 조정하겠습니다.