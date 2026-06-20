---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersCountResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCounts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NzI0IiwiaWF0IjoxNjg5MDAwMDB9.KyZ4l1X2f3Q'
  const countsWithSso: GetBannedUsersCountResponse = await getCounts(ssoToken)
  const countsWithoutSso: GetBannedUsersCountResponse = await getCounts()
  console.log(countsWithSso, countsWithoutSso)
})()
[inline-code-end]

---