## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример putCloseThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "my-company";
  const urlId: string = "post-2023-09-15";
  const ssoToken: string = "sso-abc123def";

  const resultWithSso: APIEmptyResponse = await putCloseThread(tenantId, urlId, ssoToken);
  const resultWithoutSso: APIEmptyResponse = await putCloseThread(tenantId, urlId);
})();
[inline-code-end]

---