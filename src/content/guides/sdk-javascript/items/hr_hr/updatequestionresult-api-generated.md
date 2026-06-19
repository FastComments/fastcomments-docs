---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Da |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_01";
const id: string = "question_9f2d1b";
const metaItem: MetaItem = { key: "platform", value: "web" };
const status: APIStatus = { code: 0, message: "scored" };
const updateQuestionResultBody: UpdateQuestionResultBody = {
  score: 92,
  passed: true,
  status,
  meta: [metaItem] // demonstrirano neobavezno polje
};
const result: APIEmptyResponse = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---