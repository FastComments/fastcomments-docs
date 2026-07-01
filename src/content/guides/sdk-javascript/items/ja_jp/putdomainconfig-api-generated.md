## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## レスポンス

返却: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## 例

[inline-code-attrs-start title = 'putDomainConfig 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'tenant-9f8c7b1a';
  const domainToUpdate: string = 'comments.mywebsite.org';
  const updateDomainConfigParams: UpdateDomainConfigParams = {
    enableModeration: true,
    // オプションのフィールドは省略されます、例: maxCommentLength?: number
  };
  const result: PutDomainConfigResponse = await putDomainConfig(
    tenantId,
    domainToUpdate,
    updateDomainConfigParams,
  );
  console.log(result);
}
runExample();
[inline-code-end]