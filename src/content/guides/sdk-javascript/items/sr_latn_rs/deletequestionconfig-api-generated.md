## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'deleteQuestionConfig Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-01";
const idOptional: string | undefined = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
const id: string = idOptional ?? "11111111-1111-1111-1111-111111111111";
const response: FlagCommentPublic200Response = await deleteQuestionConfig(tenantId, id);
[inline-code-end]

---