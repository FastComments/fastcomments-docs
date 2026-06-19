## Parametri

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createModeratorBody | CreateModeratorBody | Da |  |

## Odgovor

Vraća: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // neobavezni parametar (primjer)
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---