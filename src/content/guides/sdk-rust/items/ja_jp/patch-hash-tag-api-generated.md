## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | String | Yes |  |
| tenant_id | String | No |  |
| update_hash_tag_body | models::UpdateHashTagBody | No |  |

## レスポンス

戻り値: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_hash_tag_200_response.rs)

## 例

[inline-code-attrs-start title = 'patch_hash_tag の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PatchHashTagParams = PatchHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    update_hash_tag_body: Some(models::UpdateHashTagBody {
        label: Some("World News".to_string()),
        description: Some("Articles related to world events.".to_string()),
        enabled: Some(true),
    }),
};

let response: PatchHashTag200Response = patch_hash_tag(&configuration, params).await?
[inline-code-end]

---