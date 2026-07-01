## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| createTenantUserBody | CreateTenantUserBody | Yes |  |

## Odgovor

Vraća: [`CreateTenantUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUserResponse1.ts)

## Primer

[inline-code-attrs-start title = 'createTenantUser Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "a1b2c3d4-5678-90ab-cdef-1234567890ab";

const newUser: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "admin",
  // opcioni parametri
  phoneNumber: "+15551234567",
  isActive: true,
};

const result: CreateTenantUserResponse1 = await createTenantUser(tenantId, newUser);
[inline-code-end]