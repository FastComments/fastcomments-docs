## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返却: [`GetModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse1.ts)

## 例

[inline-code-attrs-start title = 'getModerator の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-001";
  const id: string = "mod-12345";

  const result: GetModeratorResponse1 = await getModerator(tenantId, id);

  const moderatorName: string | undefined = result.moderator?.name;
  console.log(moderatorName);
})();
[inline-code-end]