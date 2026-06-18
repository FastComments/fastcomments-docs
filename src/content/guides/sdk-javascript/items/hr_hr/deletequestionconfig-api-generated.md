## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'deleteQuestionConfig Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-47a9';
const id: string = 'qcfg_20260615_001';
const result: FlagCommentPublic200Response = await deleteQuestionConfig(tenantId, id);
[inline-code-end]

---