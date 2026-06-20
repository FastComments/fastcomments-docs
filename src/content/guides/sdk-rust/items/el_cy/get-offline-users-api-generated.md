Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτήν τη στιγμή σε σύνδεση. Ταξινομημένα κατά displayName.
Χρησιμοποιήστε αυτό μετά την εξάντληση του /users/online για να εμφανίσετε μια ενότητα "Μέλη".
Σελιδοποίηση με δείκτη (cursor) στο commenterName: ο server διασχίζει το μερικό {tenantId, urlId, commenterName} index από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| url_id | String | Ναι |  |
| after_name | String | Όχι |  |
| after_user_id | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'get_offline_users Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---