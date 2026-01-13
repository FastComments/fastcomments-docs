## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateNotificationBody | UpdateNotificationBody | Ja |  |
| userId | string | Nee |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateNotification Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_86a7b3";
const id: string = "notif_20260112_01";
const userId: string = "moderator_42";
const updateNotificationBody: UpdateNotificationBody = {
  name: "Flagged comment alert",
  enabled: true,
  channels: ["email"],
  recipients: ["mod-team@news-site.com"],
  threshold: 1
};

(async () => {
  const result: FlagCommentPublic200Response = await updateNotification(tenantId, id, updateNotificationBody, userId);
  console.log(result);
})();
[inline-code-end]

---