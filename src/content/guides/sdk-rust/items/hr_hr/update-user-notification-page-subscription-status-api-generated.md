Omogućavanje ili onemogućavanje obavijesti za stranicu. Kada su korisnici pretplaćeni na stranicu, obavijesti se stvaraju
za nove glavne komentare, i također

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| url | String | Da |  |
| page_title | String | Da |  |
| subscribed_or_unsubscribed | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

---