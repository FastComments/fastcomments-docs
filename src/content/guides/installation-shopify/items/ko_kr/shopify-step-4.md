다음으로 `100`번째 줄로 스크롤하겠습니다:

<div class="screenshot white-bg">
    <div class="title">100번째 줄로 스크롤</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="100번째 줄로 스크롤" />
</div>

Now copy the following code snippet, which is designed **specifically for Shopify - do not use code snippets from other tutorials**:

[inline-code-attrs-start title = 'Shopify FastComments 스니펫'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

이제 커서를 `line 101` - `</div>` 바로 다음 - 에 두고 붙여넣습니다. 다음과 같아야 합니다:

<div class="screenshot white-bg">
    <div class="title">FastComments 코드 추가</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="FastComments 코드 추가" />
</div>

이제 저장할 수 있습니다:

<div class="screenshot white-bg">
    <div class="title">저장</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="저장" />
</div>

---