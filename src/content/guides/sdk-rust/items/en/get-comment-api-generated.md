## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`GetComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetCommentParams = GetCommentParams {
    tenant_id: "acme-corp-tenant".to_owned(),
    id: "news/article/2026-01-12-12345".to_owned(),
    include_replies: Some(true),
    include_deleted: Some(false),
};

let comment: GetComment200Response = get_comment(&configuration, params).await?;
[inline-code-end]
