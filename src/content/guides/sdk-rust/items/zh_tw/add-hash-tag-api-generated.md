## 參數

| 名稱 | 類型 | 是否必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 否 |  |
| create_hash_tag_body | models::CreateHashTagBody | 否 |  |

## 回應

回傳: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_hash_tag_response.rs)

## 範例

[inline-code-attrs-start title = 'add_hash_tag 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: AddHashTagParams = AddHashTagParams {
        tenant_id: Some("acme-corp-tenant".to_string()),
        create_hash_tag_body: Some(models::CreateHashTagBody {
            name: "breaking-news".to_string(),
            slug: "news/breaking".to_string(),
        }),
    };
    let response: CreateHashTagResponse = add_hash_tag(&configuration, params).await?;
    let _created_tag = response;
    Ok(())
}
[inline-code-end]

---