Προηγούμενοι σχολιαστές της σελίδας που ΔΕΝ είναι επί του παρόντος συνδεδεμένοι. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε το αφού εξαντλήσετε το /users/online για να αποδώσετε μια ενότητα "Members".
Σελιδοποίηση με δείκτη (cursor) στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName}
ξεκινώντας από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| url_id | String | Ναι |  |
| after_name | String | Όχι |  |
| after_user_id | String | Όχι |  |

## Απάντηση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]