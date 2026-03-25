## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantSuffix: string | undefined = undefined;
const tenantId: string = tenantSuffix ? `tenant-${tenantSuffix}` : 'tenant-9142a7';
const id: string = '3f9b2a44-1c2e-4d3b-9f6a-8e7c6d5b2a1f';
const result: FlagCommentPublic200Response = await deleteNotificationCount(tenantId, id);
console.log(result);
[inline-code-end]

---