---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |

## Odpowiedź

Zwraca: [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład delete_user_badge'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
    tenant_id: "acme-newsroom-tenant".to_string(),
    id: "badge-moderator-001".to_string(),
};
let include_related: Option<bool> = Some(false);
let result: ApiEmptySuccessResponse = delete_user_badge(&configuration, params).await?;
[inline-code-end]

---