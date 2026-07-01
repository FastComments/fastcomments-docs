Bulk uporabniški podatki za najemnika. Na podlagi podanih userIds se vrnejo podatki za prikaz iz User / SSOUser.  
Uporablja se v pripomočku za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili prek dogodka prisotnosti.  
Brez konteksta strani: zasebnost se izvaja enotno (zasebni profili so maskirani).

## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| ids | String | Da |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'Primer get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]