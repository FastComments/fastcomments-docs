## Parametri

| Naziv | Type | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| body | PickAPICommentUpdatableCommentFields | Da |  |
| contextUserId | string | Ne |  |
| doSpamCheck | boolean | Ne |  |
| isLive | boolean | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)