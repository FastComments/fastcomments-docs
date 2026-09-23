## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse.ts)

## 例

[inline-code-attrs-start title = 'getModerator の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerator() {
  const tenantId: string = "c9f1e2b3-4d5a-6f78-90ab-cdef12345678";
  const moderatorId: string = "mod-987654";
  const response: GetModeratorResponse = await getModerator(tenantId, moderatorId);
  const isActive: boolean | undefined = response.moderator?.isActive;
  const statusCode: number | undefined = response.status?.code;
}
[inline-code-end]

---