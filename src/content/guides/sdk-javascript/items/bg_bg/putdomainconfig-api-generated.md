## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domainToUpdate | string | Да |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Да |  |

## Отговор

Връща: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutDomainConfigResponse.ts)

## Пример

[inline-code-attrs-start title = 'putDomainConfig Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = 'tenant-9f8c7b1a';
  const domainToUpdate: string = 'comments.mywebsite.org';
  const updateDomainConfigParams: UpdateDomainConfigParams = {
    enableModeration: true,
    // незадължително поле пропуснато, напр., maxCommentLength?: number
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