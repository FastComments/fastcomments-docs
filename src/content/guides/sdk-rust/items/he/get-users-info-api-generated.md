Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Used by the comment widget to enrich users that just appeared via a presence event.  
No page context: privacy is enforced uniformly (private profiles are masked).

## Parameters

| שם | סוג | דרוש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## Response

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'דוגמת get_users_info'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]