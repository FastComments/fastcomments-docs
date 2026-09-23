Идентификује кориснички акредитив у употреби: tenant којем припада и, за OAuth токене, корисника који га је ауторизовао.  
Интеграције користе ово за тестирање везе и означавање.

## Parameters

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |

## Response

Враћа: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getMe'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]