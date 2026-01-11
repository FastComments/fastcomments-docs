FastComments SSR를 사용하려면 클라이언트가 `https://fastcomments.com/ssr/comments` 엔드포인트에서 HTML을 가져올 수 있습니다.

이는 여러 가지 방법으로 수행할 수 있습니다.

### WordPress에서

SSR은 WordPress 플러그인 버전 `3.10.2` 이후 자바스크립트가 활성화되지 않은 사용자를 위한 대체(fallback)로 기본적으로 활성화되어 있습니다.

### 웹페이지에서

이미 존재하는 애플리케이션이 있는 경우, 사용하는 언어가 PHP라고 가정하면 [다음 예제](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)로 SSR을 추가할 수 있습니다:

[inline-code-attrs-start title = 'PHP 기반 SSR 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

자바스크립트가 비활성화된 사용자에게만 SSR UI를 표시하도록 할 수도 있습니다:

[inline-code-attrs-start title = 'PHP 기반 SSR 대체 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

SSO를 사용하는 예제는 [여기](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)를 참조하세요.

### 사전 렌더링된 콘텐츠에서

저희 블로그는 빌드 시 생성되며, [Handlebars를 이용한 SSR의 좋은 예제](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51)를 제공합니다.

### 기본 매개변수

전달해야 하는 기본 매개변수는 다음과 같습니다:
- `tenantId` - 고객으로서 사용자를 식별합니다.
- `urlId` - 댓글을 로드할 페이지나 게시물을 식별하며, 댓글이 저장되는 위치를 정의합니다.
- `url` - 알림 및 관련 기능에서 댓글 스레드로 다시 링크할 때 사용됩니다.

### 사용자 지정 스타일

댓글 위젯의 SSR 버전은 자바스크립트 버전과 동일한 구조와 렌더링 엔진을 사용합니다.

따라서 자바스크립트 댓글 위젯에 적용되는 모든 사용자 지정 스타일은 SSR에도 적용됩니다.

### 참고

SSR의 경우 렌더된 컨테이너의 높이를 제어할 자바스크립트가 없습니다. 브라우저에서는 긴 토론의 경우 세로 스크롤바가 표시될 수 있습니다.

따라서 필요에 따라 이를 조정해야 합니다.

---