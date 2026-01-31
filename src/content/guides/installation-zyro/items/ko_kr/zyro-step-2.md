이제 위젯 코드를 추가해봅시다.

아래 코드를 복사하세요. [fastcomments.com](https://fastcomments.com)에 로그인되어 있는지 확인하고, 로그인되어 있지 않다면 이 페이지를 새로고침하여 코드가 계정 정보로 자동 채워지도록 하세요. 그렇지 않으면 데모 코드가 표시됩니다.

이제 코드를 복사합니다:

[inline-code-attrs-start title = 'Zyro 댓글 코드'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

이제 사이트 빌더로 돌아가서 `Enter code`를 클릭하세요:

<div class="screenshot white-bg">
    <div class="title">코드 입력</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="코드 입력" />
</div>

### 참고!

위 코드를 사용해야 하며 다른 문서의 코드 스니펫을 사용하면 안 됩니다. 이 스니펫은 Zyro에 맞게 특별히 제작되었습니다.

다음과 같이 빈 상태로 보이는 항목이 있을 것입니다. 이는 정상입니다. 위젯이 있어야 하는 영역에 마우스를 올려보세요:

<div class="screenshot white-bg">
    <div class="title">위젯 코드 추가됨</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="위젯 코드 추가됨" />
</div>

이제 위젯의 크기를 원하는 대로 드래그하면 나타나는 것을 볼 수 있습니다:

<div class="screenshot white-bg">
    <div class="title">크기 조정</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="크기 조정" />
</div>

...이제 미리보기하고 저장하세요!