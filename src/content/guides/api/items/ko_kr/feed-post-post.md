[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 `FeedPost`를 생성합니다. 모든 게시물에는 작성자가 있으므로 `fromUserId`가 필요하며, 이는 계정에 존재하는 FastComments 또는 SSO 사용자의 ID여야 합니다.

[inline-code-attrs-start title = 'FeedPost 생성 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** 현재 브라우저에서 열려 있는 피드에 게시물을 푸시합니다. 기본값은 false입니다. **/
    isLive?: boolean
    /** 저장하기 전에 스팸 엔진을 통해 게시물을 실행합니다. 기본값은 false입니다. **/
    doSpamCheck?: boolean
    /** doSpamCheck의 일부로 실행되는 중복 콘텐츠 검사를 건너뜁니다. 기본값은 false입니다. **/
    skipDupCheck?: boolean
    /** 최대 256자. 실시간 청취자에게 에코되어 클라이언트가 자신의 방송을 무시할 수 있습니다. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** 필수. FastComments 또는 SSO 사용자 ID. **/
    fromUserId: string
    title?: string
    /** HTML. 저장 시 정제됩니다. **/
    contentHTML?: string
    /** 사용자에게서 가져온 표시 이름을 재정의합니다. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** 실패 시 포함됩니다. **/
    reason?: string
    feedPost?: FeedPost; // 성공 시 전체 생성된 게시물을 반환합니다.
}
[inline-code-end]