## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template200response.nim)

## Örnek

[inline-code-attrs-start title = 'getEmailTemplate Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email-01")
if response.isSome:
  let template = response.get()
  echo "Template ID: ", template.id
  echo "Subject: ", template.subject
  echo "Body: ", template.body
[inline-code-end]

---