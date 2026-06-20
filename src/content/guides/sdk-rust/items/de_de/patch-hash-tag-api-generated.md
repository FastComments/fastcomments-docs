## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | String | Ja |  |
| tenant_id | String | Nein |  |
| update_hash_tag_body | models::UpdateHashTagBody | Nein |  |

## Antwort

Gibt zurück: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## Beispiel

[inline-code-attrs-start title = 'patch_hash_tag Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let cfg: &configuration::Configuration = &configuration;
let body: models::UpdateHashTagBody = Default::default();
let params: PatchHashTagParams = PatchHashTagParams {
    tag: "news/article".to_string(),
    tenant_id: Some("acme-corp-tenant".to_string()),
    update_hash_tag_body: Some(body),
};
let response: UpdateHashTagResponse = patch_hash_tag(cfg, params).await?;
[inline-code-end]

---