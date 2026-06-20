Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergave-informatie van User / SSOUser.
Wordt gebruikt door de commentaar-widget om gebruikers te verrijken die zojuist zijn verschenen via een presence-gebeurtenis.
Geen pagina-context: privacy wordt uniform gehandhaafd (privéprofielen worden afgeschermd).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| ids | String | Ja |  |

## Response

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_users_info Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---