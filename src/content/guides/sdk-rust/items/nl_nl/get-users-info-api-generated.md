Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergave‑informatie van User / SSOUser.  
Gebruikt door de commentaarwidget om gebruikers die net verschenen via een presence‑event te verrijken.  
Geen paginacontext: privacy wordt uniform afgedwongen (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| ids | String | Ja |  |

## Response

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'Voorbeeld get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]