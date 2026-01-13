## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| direction | CreateVoteDirectionEnum | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

回傳: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## 範例

[inline-code-attrs-start title = 'createVote 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-812';
  const commentId: string = '5e8f8b7a-3d4b-4f1b-9a2e-1c9f2d6a7b8c';
  const direction: CreateVoteDirectionEnum = CreateVoteDirectionEnum.UP;
  const anonUserId: string = 'anon-84b9c2d';
  const voteResult: VoteComment200Response = await createVote(tenantId, commentId, direction, undefined, anonUserId);
  console.log(voteResult);
})();
[inline-code-end]

---