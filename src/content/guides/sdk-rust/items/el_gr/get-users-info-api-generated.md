Μαζικές πληροφορίες χρήστη για έναν ενοικιαστή. Δοθέντες οι userIds, επιστρέφονται πληροφορίες εμφάνισης από User / SSOUser.  
Χρησιμοποιείται από το widget σχολίων για τον εμπλουτισμό των χρηστών που μόλις εμφανίστηκαν μέσω ενός γεγονότος παρουσίας.  
Δεν υπάρχει πλαίσιο σελίδας: η ιδιωτικότητα επιβάλλεται ομοιόμορφα (τα ιδιωτικά προφίλ καλύπτονται).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ids | String | Yes |  |

## Response

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## Example

[inline-code-attrs-start title = 'get_users_info Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]