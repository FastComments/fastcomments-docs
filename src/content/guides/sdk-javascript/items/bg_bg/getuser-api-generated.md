## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Response

Връща: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Пример

[inline-code-attrs-start title = 'getUser Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-publishing-42";
const userIdOptional: string | undefined = "user_9d7b4c"; // може да е undefined в някои потоци (незадължително)
const id: string = userIdOptional ?? "user_default_0001";
const result: GetUser200Response = await getUser(tenantId, id);
console.log(result);
[inline-code-end]

---