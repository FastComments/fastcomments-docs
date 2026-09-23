[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

从其评论中移除投票以及所有已投的票。评论本身保持不变。

删除评论会同时删除其投票和投票记录，因此只有在想保留评论时才需要使用此操作。

[inline-code-attrs-start title = '投票删除 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '投票删除请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '投票删除响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]