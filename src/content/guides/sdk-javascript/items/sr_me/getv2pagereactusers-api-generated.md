## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactUsersResponse.ts)

## Пример

[inline-code-attrs-start title = 'getV2PageReactUsers Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fc_tenant_7b4c9d1";
const rawUrlId: string | undefined = undefined; // може доћи из параметара руте
const urlId: string = rawUrlId ?? "page-home-9a3f2b";
const id: string = "user_823b5c";

const response: GetV2PageReactUsersResponse = await getV2PageReactUsers(tenantId, urlId, id);
[inline-code-end]

---