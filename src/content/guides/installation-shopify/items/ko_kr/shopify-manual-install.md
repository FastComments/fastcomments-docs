[Shopify App Store 앱](https://apps.shopify.com/fastcomments)을 설치할 수 없는 경우에도 테마를 편집하여 FastComments를 추가할 수 있습니다. 이 방법은 이미 소유하고 있는 FastComments 테넌트를 연결하거나, 앱 설치가 불가능한 Shopify 스토어프론트에 임베드할 때 유용합니다.

앱 기반 설치가 대부분의 스토어에서 지원되는 방법입니다. 앱이 적합하지 않은 경우에만 이 방법을 사용하세요.

### 1단계: Shopify 기본 댓글 비활성화

Shopify 관리자 페이지에서 **Blog posts > Manage blogs**로 이동하여 각 블로그를 열고 오른쪽 패널에서 **Comments are disabled**를 설정합니다. 저장합니다.

이렇게 하면 Shopify의 내장 댓글 기능이 FastComments와 함께 표시되지 않습니다.

### 2단계: 블로그 테마 템플릿 열기

Shopify 관리자 페이지에서:

1. **Online Store > Themes**로 이동합니다.
2. 현재 테마에서 **Actions > Edit code**를 클릭합니다.
3. 왼쪽 파일 브라우저에서 **Sections**를 열고 `main-article.liquid`를 클릭합니다.

이것은 Shopify가 개별 블로그 아티클을 렌더링하는 데 사용하는 템플릿입니다.

### 3단계: FastComments 스니펫 붙여넣기

`main-article.liquid`의 대략 100번째 줄, 아티클 본문의 닫는 `</div>` 바로 뒤로 스크롤합니다. 다음 스니펫을 붙여넣습니다:

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

`"demo"`를 [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret)에서 확인한 본인의 Tenant ID로 교체합니다. **Save**를 클릭합니다.

### 4단계: 스토어 도메인 승인

라이브 스토어에서 블로그 게시물을 엽니다. 댓글 위젯 대신 승인 오류가 표시되면, FastComments가 해당 스토어가 이 테넌트를 사용할 수 있도록 허용 목록에 등록해야 합니다. [도메인 오류](/guide-installation-shopify.html#shopify-domain-errors)를 참조하세요.

### 다른 페이지에 FastComments 추가하기

동일한 스니펫은 제품 페이지, 커스텀 페이지, 홈 페이지를 포함한 모든 Liquid 템플릿에서 작동합니다. 댓글을 표시하려는 위치에 붙여넣고, 페이지별로 고정 식별자를 사용하려면 `urlId`를 조정하세요 (예: 제품 템플릿에서 `urlId: "{{ product.id }}"` 사용).