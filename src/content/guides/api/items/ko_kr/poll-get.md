[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

댓글에 연결된 설문조사를 현재 투표 수와 함께 읽어옵니다.

설문조사는 댓글 API를 통해 댓글 자체에도 반환되므로, 전체 댓글이 아니라 결과만 필요할 때 이 엔드포인트를 사용하십시오.

[inline-code-attrs-start title = 'Poll 가져오기 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Poll 가져오기 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll 가져오기 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

설문조사가 없는 댓글, 삭제된 댓글, 존재하지 않는 댓글 ID 모두 `poll-not-found`와 함께 동일한 방식으로 응답합니다.