The **FastComments** 블록은 주요 댓글 위젯입니다. 블로그 게시물 템플릿, 제품 템플릿 또는 토론 스레드나 실시간 채팅이 필요한 다른 페이지에 추가하세요.

### Add the block

1. Shopify 테마 편집기(**Online Store > Themes > Customize**)를 엽니다.
2. 댓글을 달고 싶은 템플릿을 선택합니다: **Blog post**, **Product**, 또는 다른 페이지나 섹션 템플릿.
3. 댓글을 표시할 섹션에서 **Add block**을 클릭합니다.
4. **Apps** 아래에서 **FastComments**를 선택합니다.
5. **Save**를 클릭합니다.

블록은 즉시 나타납니다. 입력해야 할 Tenant ID가 없습니다; 앱을 설치하면 스토어의 tenant가 자동으로 연결됩니다.

### Settings

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | 블록이 렌더링할 FastComments tenant를 재정의합니다. 비워 두면 스토어에 자동 구성된 tenant를 사용합니다. 수동 tenant ID는 fastcomments.com/auth/my-account/api-secret에서 찾을 수 있습니다. | (빈값) |
| SSO | 댓글 작성 전에 방문자를 해당 Shopify 고객 계정으로 자동 로그인시킵니다. 자세한 내용은 [Shopify 고객 자동 로그인](/guide-installation-shopify.html#shopify-sso)을 참조하세요. | On |
| Commenting Style | 중첩된 답글과 투표를 지원하는 **Threaded** 또는 실시간 채팅 피드를 위한 **Streaming**을 선택합니다. | Threaded |
| Custom URL ID | 자동으로 감지된 페이지 식별자를 재정의합니다. 두 개의 URL이 동일한 댓글 스레드를 공유하게 하려면 이 설정을 사용하세요. | (auto-detected) |

### How the page identifier is chosen

각 댓글 스레드는 URL ID로 키가 지정됩니다. 블록이 하나를 자동으로 선택합니다:

- **Blog post template:** `shopify-article-{article.id}`, 이는 슬러그나 제목 변경에도 안정적입니다.
- **Product template:** `shopify-product-{product.id}`, 이는 슬러그나 제목 변경에도 안정적입니다.
- **Other templates:** 요청 경로(request path).

**Custom URL ID**를 설정하면 해당 값이 대신 사용됩니다. 동일한 Custom URL ID를 여러 블록에서 사용하면(예: 제품 페이지의 지역화된 버전에서) 하나의 댓글 스레드를 공유할 수 있습니다.

### Threaded vs Streaming

**Threaded**는 기본값입니다. 방문자들이 서로에게 답글을 달고, 투표하며, 관리자 도구가 예상대로 작동합니다. 블로그 게시물 및 제품 리뷰에 적합합니다.

**Streaming**은 스레딩을 제거하고 새 댓글을 게시되는 즉시 실시간으로 표시합니다. 채팅 피드와 유사합니다. 제품 출시, 라이브 이벤트 및 커뮤니티 페이지에 적합합니다.

### Multiple blocks on the same page

동일한 템플릿에 블록을 여러 번 추가할 수 있습니다. 예를 들어, 제품 페이지 상단에 리뷰 요약을, 하단에 FastComments 블록을 둘 수 있습니다. 블록들은 URL ID를 공유하므로 요약은 아래의 댓글을 반영합니다.

### Tips

- 블록은 tenant를 찾을 수 없을 때 테마 편집기 미리보기에서 노란색 알림과 함께 숨겨집니다. 라이브 스토어에서 이 알림이 나타나면 FastComments 앱을 재설치하세요.
- 제품 페이지의 경우 FastComments 블록은 제품 리뷰 위젯 역할도 합니다. 페이지 상단에 별점 요약을 추가하려면 **FastComments - Reviews Summary**와 함께 사용하세요.