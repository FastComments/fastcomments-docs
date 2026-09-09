---
Webhooks can also be managed through the REST API. This is how integrations such as Zapier subscribe
to comment events without touching the dashboard, and it follows the REST Hooks pattern: subscribe,
receive events, unsubscribe.

API subscriptions live alongside the webhooks configured in the dashboard. A comment event is delivered
to the dashboard webhook for its domain and to every API subscription that matches, each as its own
delivery. There is no limit of one subscriber per event.

## אימות

Every request needs your API Key in the `x-api-key` header (or the `API_KEY` query parameter) and
your tenant ID in the `tenantId` query parameter. Both are shown on the API Secret page in the dashboard.

## הרשמה

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| שדה | נדרש | תיאור |
|-------|----------|-------------|
| `url` | Yes | כתובת URL מוחלטת http או https. |
| `event` | Yes | `comment-created`, `comment-updated` או `comment-deleted`. |
| `domain` | No | דומיין מהגדרות החשבון שלך. ברירת המחדל היא `*`, שמקבל אירועים מכל דומיין. |
| `method` | No | `POST` (default), `PUT` or `DELETE`. |

The response contains the subscription:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Subscribing the same URL to the same event and domain again returns the existing subscription rather
than creating a duplicate, so a client can safely retry. Each tenant can have up to 50 API subscriptions.

## רשימה

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Returns every webhook for the tenant, including those managed in the dashboard (`"source": "dashboard"`).
Filter with `event`, `domain` or `source`.

## ביטול מנוי

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Deleting a subscription also discards any events still queued for it. Only subscriptions created
through the API can be deleted this way. Dashboard webhooks are edited on the Webhooks page.

## מטענים וחתימה

Deliveries use the same payload as dashboard webhooks (see Data Structures) and are signed with the same
HMAC scheme (see Security & API Tokens). API subscriptions never receive the legacy `token` header, so
verify the `X-FastComments-Signature` header instead.

## תגובה עם 410 Gone

If an API subscription's endpoint responds with HTTP `410 Gone`, FastComments treats that as an
unsubscribe: the subscription is deleted along with its queued events, and no further deliveries are
attempted. Webhooks configured in the dashboard are never deleted automatically; for them a 410 is an
ordinary failure. Any other failure status is retried and eventually disables the webhook, as described
in How it Works & Handling Retries.

## לוח הבקרה

API subscriptions are listed on the Webhooks page under the domain they were created for, where an
administrator can disable, re-enable or delete them.