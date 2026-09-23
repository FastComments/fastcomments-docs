## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Відповідь

Повертає: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## Приклад

[inline-code-attrs-start title = 'createTenantUser Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function addTenantUser() {
  const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
  const digestFreq: DigestEmailFrequency = 'daily';
  const body: CreateTenantUserBody = {
    email: 'jane.doe@example.com',
    firstName: 'Jane',
    lastName: 'Doe',
    role: 'admin',
    digestEmailFrequency: digestFreq,
    phoneNumber: '+15551234567' // необов'язковий
  };
  const response: CreateTenantUserResponse = await createTenantUser(tenantId, body);
  console.log(response);
}
addTenantUser();
[inline-code-end]

---