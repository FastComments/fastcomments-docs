## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getModerator Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_eu_4f8d2b9e";
const maybeModeratorId: string | undefined = "mod_91c3b7a2"; // isteğe bağlı kaynak (undefined olabilir)
const moderator: GetModerator200Response = await getModerator(tenantId, maybeModeratorId!);
[inline-code-end]

---