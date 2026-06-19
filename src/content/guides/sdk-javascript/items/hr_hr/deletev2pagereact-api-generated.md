## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteV2PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface APIStatus { code: number; message?: string }
interface CreateV1PageReact { id: string; pageUrl: string; userId?: string; status?: APIStatus; deleted?: boolean }

const tenantId: string = 'tenant_prod_8621'
const urlId: string = 'page_home_001'
const id: string = 'react_5f9b1c3a'

const result: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, id)
const statusCode: number | undefined = result.status?.code
console.log('Deleted reaction id:', result.id, 'statusCode:', statusCode)
[inline-code-end]

---