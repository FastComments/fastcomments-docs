If you can't install the [Shopify App Store app](https://apps.shopify.com/fastcomments), you can still add FastComments by editing your theme. This path is useful when you want to wire up a FastComments tenant you already own, or when you're embedding on a Shopify storefront where the app isn't an option.

앱 기반 설치가 대부분의 스토어에 권장되는 방법입니다. 앱이 적합하지 않을 때만 이 방법을 사용하세요.

### 1단계: Shopify의 기본 댓글 비활성화

Shopify 관리자에서 **Blog posts > Manage blogs**로 이동하고 각 블로그를 열어 오른쪽 패널에서 **Comments are disabled**로 설정하세요. 저장.

이렇게 하면 Shopify의 기본 댓글이 FastComments와 함께 표시되는 것을 중지합니다.

### 2단계: 블로그 테마 템플릿 열기

Shopify 관리자에서:

1. **Online Store > Themes**로 이동합니다.
2. 현재 테마 아래에서 **Actions > Edit code**를 클릭합니다.
3. 왼쪽 파일 브라우저에서 **Sections**를 열고 `main-article.liquid`를 클릭합니다.

이 템플릿은 Shopify가 단일 블로그 게시물을 렌더링할 때 사용하는 템플릿입니다.

### 3단계: FastComments 스니펫 붙여넣기

`main-article.liquid`의 대략 100번째 줄, 게시물 본문의 닫는 `</div>` 바로 다음으로 스크롤하세요. 다음 스니펫을 붙여넣으세요:

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

Replace `"demo"` with your own Tenant ID from [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). **저장**을 클릭하세요.

### 4단계: 상점 도메인 승인

라이브 스토어에서 블로그 게시물을 엽니다. 댓글 위젯 대신 권한 오류가 표시되면 FastComments가 해당 테넌트를 이 스토어에서 사용할 수 있도록 허용해야 합니다. 자세한 내용은 [도메인 오류](/guide-installation-shopify.html#shopify-domain-errors)를 참조하세요.

### 다른 페이지에 FastComments 추가하기

동일한 스니펫은 제품 페이지, 커스텀 페이지, 홈 페이지를 포함한 모든 Liquid 템플릿에서 작동합니다. 댓글을 표시하려는 위치에 붙여넣고, 페이지별로 안정적인 식별자가 필요하면 `urlId`를 조정하세요(예: 제품 템플릿에서 `urlId: "{{ product.id }}"`).

---