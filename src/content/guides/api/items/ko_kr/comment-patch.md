[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 단일 댓글을 업데이트하는 기능을 제공합니다.

참고:

- 필요에 따라 이 API로 댓글 위젯을 "라이브"로 업데이트할 수 있습니다(이 경우 기본 `creditsCost`가 `1`에서 `2`로 증가합니다).
  - 이는 댓글을 페이지 간에 "라이브"로 마이그레이션(예: `urlId` 변경)하는 데 사용할 수 있습니다.
  - 마이그레이션은 페이지를 미리 계산하므로 추가로 `2` 크레딧이 소모됩니다(이 작업은 CPU 집약적입니다).
- 생성 API와 달리, 이 API는 이메일이 제공되어도 우리 시스템에 사용자 객체를 자동으로 생성하지 않습니다.
- 이 API로 업데이트된 댓글은 원하면 스팸 검사 대상이 될 수 있습니다.
- Customization Rule 관리자 페이지에서 구성된 최대 댓글 길이와 같은 설정이 여기에도 적용됩니다.
- 사용자가 자신의 댓글 텍스트를 업데이트하도록 허용하려면 요청 본문에 `comment`만 지정하면 됩니다. 우리는 결과 `commentHTML`을 생성합니다.
  - `comment`와 `commentHTML`을 둘 다 정의하면 HTML을 자동으로 생성하지 않습니다.
  - 사용자가 새 텍스트에 멘션이나 해시태그를 추가하면 `POST` API와 동일하게 처리됩니다.
- 댓글의 `commenterEmail`을 업데이트할 때는 `userId`도 함께 지정하는 것이 좋습니다. 그렇지 않으면 해당 이메일을 가진 사용자가 귀하의 테넌트에 속해 있는지 확인해야 하며, 그렇지 않으면 요청이 실패합니다.  


[inline-code-attrs-start title = '최소 댓글 PATCH cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = '댓글 PATCH 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** 업데이트를 수행하는 사용자입니다. 필요하면 사용자가 댓글을 편집할 수 있는지 확인하는 데 사용할 수 있습니다. **/
    contextUserId?: string
	/** 새 댓글이 스팸처럼 보이는지 검사할까요? **/
    doSpamCheck?: 'true' | 'false'
	/** 동일한 urlId를 가진 댓글 위젯 인스턴스를 보는 사용자에게 댓글이 "라이브"로 보일지 여부입니다. 참고: 크레딧 비용이 1에서 2로 두 배가 됩니다. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 PATCH 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---