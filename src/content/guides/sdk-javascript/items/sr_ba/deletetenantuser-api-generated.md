## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| deleteComments | string | Ne |  |
| commentDeleteMode | string | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '4f3a9b2e-1c6a-4f7b-9e2a-0b8d6f1c2a3e';
const userId: string = '9d2f7b3a-5c4e-48a2-b1f0-6c7d8e9f0a12';
const deleteComments: string = 'true';
const commentDeleteMode: string = 'permanent';
const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, userId, deleteComments, commentDeleteMode);
[inline-code-end]