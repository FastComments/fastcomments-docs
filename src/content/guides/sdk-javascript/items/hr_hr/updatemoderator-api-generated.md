## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateModeratorBody | UpdateModeratorBody | Da |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";
const moderatorId: string = "mod_12345";

const updateBody: UpdateModeratorBody = {
  isActive: true,
  // role?: string je opcionalan i ovdje je izostavljen
};

const result: APIEmptyResponse = await updateModerator(tenantId, moderatorId, updateBody);
[inline-code-end]