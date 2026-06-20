## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tag | String | 是 |  |
| tenant_id | String | 否 |  |
| delete_hash_tag_request_body | models::DeleteHashTagRequestBody | 否 |  |

## 回應

回傳: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_hash_tag 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteHashTagParams = DeleteHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    delete_hash_tag_request_body: Some(DeleteHashTagRequestBody {}),
};
let response: ApiEmptyResponse = delete_hash_tag(&configuration, params).await?;
[inline-code-end]