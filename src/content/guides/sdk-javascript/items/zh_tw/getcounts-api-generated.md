## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| sso | string | 否 |  |

## 回應

回傳: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersCountResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCounts 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NzI0IiwiaWF0IjoxNjg5MDAwMDB9.KyZ4l1X2f3Q'
  const countsWithSso: GetBannedUsersCountResponse = await getCounts(ssoToken)
  const countsWithoutSso: GetBannedUsersCountResponse = await getCounts()
  console.log(countsWithSso, countsWithoutSso)
})()
[inline-code-end]

---