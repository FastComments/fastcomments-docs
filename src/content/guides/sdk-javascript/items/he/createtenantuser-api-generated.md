## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## תגובה

מחזיר: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'createTenantUser דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    phoneNumber: '+15551234567' // אופציונלי
  };
  const response: CreateTenantUserResponse = await createTenantUser(tenantId, body);
  console.log(response);
}
addTenantUser();
[inline-code-end]