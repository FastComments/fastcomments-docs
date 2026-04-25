[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 단일 댓글을 업데이트할 수 있는 기능을 제공합니다.

참고:

- 이 API는 원하면 댓글 위젯을 "실시간"으로 업데이트할 수 있습니다(기본 `creditsCost`가 `1`에서 `2`로 증가합니다).
  - 이를 통해 댓글을 페이지 간에 "실시간"으로 이동(예: `urlId` 변경)할 수 있습니다.
  - 페이지가 미리 계산되므로 마이그레이션은 추가로 `2` 크레딧이 소모되며 CPU 집약적입니다.
- 생성 API와 달리, 이 API는 이메일이 제공되더라도 우리 시스템에 사용자 객체를 자동으로 생성하지 않습니다.
- 이 API로 업데이트된 댓글은 원하면 스팸 검사 대상이 될 수 있습니다.
- 최대 댓글 길이와 같은 구성은 Customization Rule 관리자 페이지에서 구성된 경우 여기에도 적용됩니다.
- 사용자가 자신의 댓글 텍스트를 업데이트할 수 있도록 하려면 요청 본문에 단순히 `comment`를 지정하면 됩니다. 우리는 결과 `commentHTML`을 생성합니다.
  - `comment`와 `commentHTML`을 모두 정의하면 HTML을 자동으로 생성하지 않습니다.
  - 사용자가 새 텍스트에 멘션이나 해시태그를 추가하면 `POST` API와 같이 처리됩니다.
- 댓글의 `commenterEmail`을 업데이트할 때는 `userId`도 함께 지정하는 것이 가장 좋습니다. 그렇지 않으면 해당 이메일을 가진 사용자가 귀하의 테넌트에 속하는지 확인해야 하며, 그렇지 않으면 요청이 실패합니다.  
- 대상 댓글이 잠겨 있는 경우(`isLocked: true`) 요청은 `code: 'locked'`로 거부됩니다. 먼저 댓글의 잠금을 해제하고 업데이트한 다음 필요하면 다시 잠그십시오.


[inline-code-attrs-start title = '최소 댓글 PATCH cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** 업데이트를 수행하는 사용자. 필요하면 이 값을 사용해 댓글을 수정할 수 있는지 확인할 수 있습니다.  **/
    contextUserId?: string
	/** 새 댓글이 스팸으로 보이는지 검사할까요?  **/
    doSpamCheck?: 'true' | 'false'
	/** 같은 urlId를 사용하는 댓글 위젯 인스턴스에서 사용자가 댓글을 "실시간"으로 보게 할지 여부. NOTE: 크레딧 비용이 1에서 2로 두 배가 됩니다. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 PATCH 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

---