다음으로 `100`번째 줄로 스크롤합니다:

<div class="screenshot white-bg">
    <div class="title">100번째 줄로 스크롤</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="100번째 줄로 스크롤" />
</div>

이제 다음 코드 스니펫을 복사합니다. 이 스니펫은 **Shopify 전용으로 설계되었으므로 다른 튜토리얼의 코드 스니펫을 사용하지 마십시오**:

[inline-code-attrs-start title = 'Shopify용 FastComments 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

이제 커서를 `101`번째 줄 - `</div>` 바로 다음 - 에 놓고 붙여넣습니다. 다음과 같은 모습이어야 합니다:

<div class="screenshot white-bg">
    <div class="title">FastComments 코드 추가</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="FastComments 코드 추가" />
</div>

이제 저장합니다:

<div class="screenshot white-bg">
    <div class="title">저장</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="저장" />
</div>