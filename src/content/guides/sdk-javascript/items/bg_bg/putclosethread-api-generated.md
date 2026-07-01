## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| urlId | string | Да |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`PutCloseThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutCloseThreadResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за putCloseThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function closeThreadDemo(): Promise<void> {
  const urlId: string = "article-2023-09-15";
  const tenantId: string = "tenant-42";
  const sso: string = "sso-token-xyz";

  const response: PutCloseThreadResponse = await putCloseThread(urlId, tenantId, sso);
  console.log(response);
}

closeThreadDemo();
[inline-code-end]

---