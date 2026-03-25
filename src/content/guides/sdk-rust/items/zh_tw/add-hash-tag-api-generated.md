## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 否 |  |
| create_hash_tag_body | models::CreateHashTagBody | 否 |  |

## 回應

回傳: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/add_hash_tag_200_response.rs)

## 範例

[inline-code-attrs-start title = 'add_hash_tag 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: AddHashTagParams = AddHashTagParams {
    tenant_id: Some("acme-corp-tenant".to_string()),
    create_hash_tag_body: Some(models::CreateHashTagBody {
        tag: "breaking-news".to_string(),
        display_name: Some("Breaking News".to_string()),
        description: Some("Articles covering breaking news events".to_string()),
        enabled: Some(true),
    }),
};

let response: AddHashTag200Response = add_hash_tag(&configuration, params).await?;
[inline-code-end]

---