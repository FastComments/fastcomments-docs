---
이제 커스텀 HTML 블록을 추가했으므로, FastComments 위젯 코드를 추가할 수 있습니다.

**다음 코드는 Godaddy용입니다. 다른 튜토리얼의 코드를 사용하지 마세요. 이 코드는 Godaddy 전용입니다.**

다음 코드를 복사하세요:

[inline-code-attrs-start title = 'Godaddy 댓글 코드 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

이 코드 스니펫은 Godaddy와 호환되도록 설계되었으며, 블로그 게시물에서만 표시되고 홈페이지에는 표시되지 않습니다.

이제 코드를 앞서 언급한 `Step One`의 `Custom Code` 영역에 붙여넣으세요.

<div class="screenshot white-bg">
    <div class="title">코드 복사 및 붙여넣기</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Copy and Paste The Code" />
</div>

오른쪽 상단에서 'Done'을 클릭하세요:

<div class="screenshot white-bg">
    <div class="title">완료 클릭</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Click Done" />
</div>

2단계는 여기까지입니다!

---