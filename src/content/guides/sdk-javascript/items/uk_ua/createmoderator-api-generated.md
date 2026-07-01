## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createModeratorBody | CreateModeratorBody | Так |  |

## Response

Повертає: [`CreateModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse1.ts)

## Example

[inline-code-attrs-start title = 'Приклад createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const moderatorPayload: CreateModeratorBody = {
    name: "Alice Johnson",
    email: "alice.johnson@example.com"
    // необов’язкові поля, такі як description, опущені
  };
  const result: CreateModeratorResponse1 = await createModerator(tenantId, moderatorPayload);
  console.log(result);
}

runExample();
[inline-code-end]