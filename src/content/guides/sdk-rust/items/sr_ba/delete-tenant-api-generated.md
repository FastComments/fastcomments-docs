## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| sure | String | Не |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'delete_tenant Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteTenantParams = DeleteTenantParams {
    tenant_id: String::from("acme-corp-tenant"),
    id: String::from("acme-corp-tenant-001"),
    sure: Some(String::from("confirm-delete")),
};
let response: FlagCommentPublic200Response = delete_tenant(&configuration, params).await?;
[inline-code-end]

---