## Parametri

| Ime | Vrsta | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createModeratorBody | CreateModeratorBody | Da |  |

## Odgovor

Vrne: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // neobvezen parameter prikazan
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---