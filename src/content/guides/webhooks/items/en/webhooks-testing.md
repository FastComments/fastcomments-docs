In the Webhooks admin there are `Send Test Payload` buttons for each event type (Create, Update, Delete). The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

## Verifying Payloads

When testing your webhook integration, verify the incoming requests include the following headers:

1. **`token`** - Your API Secret
2. **`X-FastComments-Timestamp`** - Unix timestamp (seconds)
3. **`X-FastComments-Signature`** - HMAC-SHA256 signature

Use the HMAC signature verification to ensure payloads are authentic.

## Testing Tools

You can use tools like [webhook.site](https://webhook.site) or [ngrok](https://ngrok.com) to inspect incoming webhook payloads during development.

## Event Types

- **Create Event**: Triggered when a new comment is created. Default method: PUT
- **Update Event**: Triggered when a comment is edited. Default method: PUT
- **Delete Event**: Triggered when a comment is deleted. Default method: DELETE

Each event includes the full comment data in the request body (see [Data Structures](/guides/webhooks/webhooks-structures) for the payload format).
