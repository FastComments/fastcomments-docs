## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tag | String | Да |  |
| tenant_id | String | Не |  |
| update_hash_tag_body | models::UpdateHashTagBody | Не |  |

## Одговор

Враћа: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_hash_tag_200_response.rs)

## Пример

[inline-code-attrs-start title = 'patch_hash_tag Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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