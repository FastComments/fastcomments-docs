## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createModeratorBody | CreateModeratorBody | Так |  |

## Response

Повертає: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Example

[inline-code-attrs-start title = 'Приклад createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const moderatorPayload: CreateModeratorBody = {
  userId: "user_9876",
  // необов'язкове поле; можна опустити, якщо не потрібно
  notes: "Temporary moderator for event"
};

const response: CreateModeratorResponse = await createModerator(tenantId, moderatorPayload);
[inline-code-end]