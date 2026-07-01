## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| addDomainConfigParams | AddDomainConfigParams | Да |  |

## Ответ

Возвращает: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример addDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const config: AddDomainConfigParams = {
    domain: 'myblog.example.com',
    enabled: true,
    // описание является необязательным и здесь опущено
  };
  const response: AddDomainConfigResponse = await addDomainConfig(tenantId, config);
  console.log(response);
})();
[inline-code-end]