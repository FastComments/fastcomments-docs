## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| createAPIPageData | CreateAPIPageData | Nee |  |

## Respons

Geeft terug: [`Option[AddPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_page_api_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'addPage Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
var createData: CreateAPIPageData
createData.url = "news/nim-4-release"
createData.title = "Nim 4 Release Coverage"
createData.path = "/news/nim-4-release"
createData.isEnabled = true
createData.tags = @["nim", "release"]
createData.description = "Coverage of Nim 4 release"

let (response, httpResponse) = client.addPage(tenantId = "my-tenant-123", createAPIPageData = createData)

if response.isSome:
  let pageResp = response.get()
  echo pageResp
[inline-code-end]

---