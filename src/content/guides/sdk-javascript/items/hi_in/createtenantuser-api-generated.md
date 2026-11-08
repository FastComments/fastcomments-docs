## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createTenantUserBody | CreateTenantUserBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createTenantUser उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    phoneNumber: '+15551234567' // वैकल्पिक
  };
  const response: CreateTenantUserResponse = await createTenantUser(tenantId, body);
  console.log(response);
}
addTenantUser();
[inline-code-end]