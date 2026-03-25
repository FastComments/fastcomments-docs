## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org-82479";
const questionId: string = "q-6a3d2e1f";
const includeArchived?: boolean = false; // opcioni parametar koji demonstrira alternativni odabir cilja
const targetId: string = includeArchived ? "q-archived-112233" : questionId;
const result: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, targetId);
[inline-code-end]

---