## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## תגובה

מחזיר: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת putDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'tenant-9f8c7b1a';
  const domainToUpdate: string = 'comments.mywebsite.org';
  const updateDomainConfigParams: UpdateDomainConfigParams = {
    enableModeration: true,
    // שדה אופציונלי הושמט, למשל, maxCommentLength?: number
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