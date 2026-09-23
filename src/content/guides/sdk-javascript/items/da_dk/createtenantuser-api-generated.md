## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createTenantUserBody | CreateTenantUserBody | Ja |  |

## Svar

Returnerer: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'createTenantUser Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    phoneNumber: '+15551234567' // valgfri
  };
  const response: CreateTenantUserResponse = await createTenantUser(tenantId, body);
  console.log(response);
}
addTenantUser();
[inline-code-end]