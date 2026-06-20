## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| userId | string | Ні |  |
| trustFactor | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[SetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_user_trust_factor_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад setTrustFactor'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.setTrustFactor(userId = "user-9876", trustFactor = "high", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTk4NzYiLCJpYXQiOjE2MjQwMDAwMDB9.signature")
if response.isSome:
  let resultObj = response.get()
  echo resultObj
else:
  echo "No response received"
[inline-code-end]

---