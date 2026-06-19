## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Antwoord

Retourneert: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikes.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getV1PageLikes Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments-tenant-8a9b3";
const urlId: string = "articles/how-to-optimize-comments-2026-06-19";
const likes: GetV1PageLikes = await getV1PageLikes(tenantId, urlId);
[inline-code-end]

---