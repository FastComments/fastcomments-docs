## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const post: { title: string; questionId?: string } = { title: 'Product feedback' };
const tenantId: string = 'acme-corp-tenant-01';
const id: string = post.questionId ?? 'q-8f3a7b2c4d9e';
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
[inline-code-end]

---