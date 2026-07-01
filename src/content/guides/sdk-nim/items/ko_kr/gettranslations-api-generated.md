## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| namespace | string | 아니오 |  |
| component | string | 아니오 |  |
| options | GetTranslationsOptions | 아니오 |  |

## 응답

반환: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## 예시

[inline-code-attrs-start title = 'getTranslations 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetTranslationsOptions()
let (maybeResp, httpResp) = client.getTranslations(namespace = "my-tenant-123", component = "news/article-title", options = opts)
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]

---