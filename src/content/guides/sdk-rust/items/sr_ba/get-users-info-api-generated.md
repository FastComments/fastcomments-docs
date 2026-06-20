---
Masovni podaci o korisnicima za tenant. Za date userIds, vraća prikazne informacije iz User / SSOUser.
Koristi ga comment widget da obogati korisnike koji su se upravo pojavili putem presence event-a.
Nema konteksta stranice: privatnost se primjenjuje jednako (privatni profili su maskirani).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## Response

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'get_users_info Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---