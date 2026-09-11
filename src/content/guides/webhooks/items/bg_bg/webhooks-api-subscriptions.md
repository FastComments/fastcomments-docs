Webhooks can also be managed through the REST API. This is how integrations such as Zapier subscribe
to comment events without touching the dashboard, and it follows the REST Hooks pattern: subscribe,
receive events, unsubscribe.

API subscriptions live alongside the webhooks configured in the dashboard. A comment event is delivered
to every webhook that matches its domain, each as its own delivery, whichever way the webhook was created.

## Удостоверяване

Every request needs your API Key in the `x-api-key` header (or the `API_KEY` query parameter) and
your tenant ID in the `tenantId` query parameter. Both are shown on the API Secret page in the dashboard.

## Абониране

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Поле | Задължително | Описание |
|-------|----------|-------------|
| `url` | Да | Абсолютен http или https URL. |
| `event` | Да | `comment-created`, `comment-updated` or `comment-deleted`. |
| `domain` | Не | Домейн от конфигурацията на вашия акаунт. По подразбиране е `*`, което получава събития за всеки домейн. |
| `method` | Не | `POST` (по подразбиране), `PUT` or `DELETE`. |

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

Абонирането на същия URL за същото събитие и домейн отново връ връща съществуващия абонамент, вместо да създава дубликат, така че клиентът може безопасно да опита отново. Всеки наемател може да има до 50 API абонамента.

## Списък

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Returns every webhook for the tenant, including those managed in the dashboard (`"source": "dashboard"`). Filter with `event`, `domain` or `source`.

## Отписване

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Deleting a subscription also discards any events still queued for it. Only subscriptions created
through the API can be deleted this way; a dashboard webhook, or an id that does not exist on your
account, answers `404` with code `not-found`. Dashboard webhooks are edited on the Webhooks page.

## Полезни данни и подписване

Deliveries use the same payload as dashboard webhooks (see Data Structures) and are signed with the same
HMAC scheme (see Security & API Tokens). API subscriptions never receive the legacy `token` header, so verify the `X-FastComments-Signature` header instead.

## Примерни полезни данни

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Returns the account's most recent comments in exactly the shape a delivery carries, so an integration can
show real sample data before the first event arrives. `event` is optional and only validated, since every
event delivers the same comment object. `limit` defaults to 3 and accepts 1 to 10. Costs 2 API credits.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Отговор с 410 Gone

If an API subscription's endpoint responds with HTTP `410 Gone`, FastComments treats that as an
unsubscribe: the subscription is deleted along with its queued events, and no further deliveries are
attempted. Webhooks configured in the dashboard are never deleted automatically; for them a 410 is an
ordinary failure. Any other failure status is retried and eventually disables the webhook, as described
in How it Works & Handling Retries.

## Табло

API subscriptions appear in the Webhooks list with the source **API**, where an administrator can edit,
disable, re-enable or delete them.