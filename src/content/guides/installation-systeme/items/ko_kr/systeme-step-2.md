이제 코드를 복사하겠습니다. 코드 스니펫의 6번째 줄에 `tenantId: "demo"`라고 적혀 있다면 FastComments 계정으로 로그인한 다음
이 페이지를 새로고침하여 복사된 코드 스니펫에 계정 ID가 반영되게 하세요.

[inline-code-attrs-start title = 'Systeme.io 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

이제 에디터에 붙여넣고 저장을 클릭하세요:

<div class="screenshot white-bg">
    <div class="title">FastComments 코드 추가</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="FastComments 코드 추가" />
</div>

... 그런 다음 사이트를 저장하세요. 그게 전부입니다!