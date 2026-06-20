## Параметри

| Име | Type | Задължително | Описание |
|------|------|----------|-------------|
| tag | String | Да |  |
| tenant_id | String | Не |  |
| update_hash_tag_body | models::UpdateHashTagBody | Не |  |

## Отговор

Връща: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_hash_tag_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за patch_hash_tag'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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