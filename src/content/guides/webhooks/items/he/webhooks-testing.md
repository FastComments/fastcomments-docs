The new and edit webhook pages have a `Send Test Payload` button that sends a request to the URL currently in the form, whether or not it has been saved. The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

## אימות Payloads

When testing your webhook integration, verify the incoming requests include the following headers:

1. **`X-FastComments-Timestamp`** - חותמת זמן Unix (שניות)
2. **`X-FastComments-Signature`** - חתימת HMAC‑SHA256

Webhooks created before the signature scheme was introduced also receive a **`token`** header containing your API Secret. New webhooks do not.

Use the HMAC signature verification to ensure payloads are authentic.

## כלי בדיקה

You can use tools like [webhook.site](https://webhook.site) or [ngrok](https://ngrok.com) to inspect incoming webhook payloads during development.

## סוגי אירועים

- **Create Event**: מתבצע כאשר נוצר תגובה חדשה.
- **Update Event**: מתבצע כאשר תגובה נערכת.
- **Delete Event**: מתבצע כאשר תגובה נמחקת.

Each webhook is tied to one event and one HTTP method (POST, PUT or DELETE). Each event includes the full comment data in the request body (see [Data Structures](/guide-webhooks.html#webhooks-structures) for the payload format).

---