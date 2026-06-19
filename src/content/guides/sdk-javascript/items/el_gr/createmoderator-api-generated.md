## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createModeratorBody | CreateModeratorBody | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // προαιρετική παράμετρος για επίδειξη
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]

---