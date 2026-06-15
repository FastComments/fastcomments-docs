## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Retourneert: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getModerator Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media-58';
const id: string = 'mod-82f3b9c1';
const moderatorResponse: GetModerator200Response = await getModerator(tenantId, id);
[inline-code-end]

---