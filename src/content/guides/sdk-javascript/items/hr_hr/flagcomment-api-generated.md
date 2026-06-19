## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vraća: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer flagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-corp_01';
const commentId: string = 'cmt_5f8d7a2b3c4e';
const anonUserId: string = 'anon_9c3a1f0e';
const response: FlagCommentResponse = await flagComment(tenantId, commentId, undefined, anonUserId);
[inline-code-end]

---