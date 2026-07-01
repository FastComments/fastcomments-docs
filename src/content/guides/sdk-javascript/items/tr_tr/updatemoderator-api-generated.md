## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateModeratorBody | UpdateModeratorBody | Evet |  |

## Yanıt

Döndürür: [`UpdateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateModeratorResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateModerator Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUpdateModerator(): Promise<void> {
    const tenantId: string = "tenant_42abc";
    const moderatorId: string = "moderator_8f9e";
    const updateBody: UpdateModeratorBody = {
        isActive: true,
        role: "admin",
        // isteğe bağlı alan
        notes: "Promoted to senior moderator"
    };
    const result: UpdateModeratorResponse = await updateModerator(tenantId, moderatorId, updateBody);
    console.log(result);
}

demoUpdateModerator();
[inline-code-end]

---