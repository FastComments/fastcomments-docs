## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetTenantManualBadgesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_manual_badges_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getManualBadges'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getManualBadges(sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")
if response.isSome:
  let badges = response.get()
  echo "Manual badges received:"
  echo badges
else:
  echo "No manual badges returned."
  echo httpResponse
[inline-code-end]

---