## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Одговор

Враћа: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerators200Response.ts)

## Пример

[inline-code-attrs-start title = 'getModerators Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-12345-prod';
const moderatorsPage1: GetModerators200Response = await getModerators(tenantId);
const moderatorsPage2: GetModerators200Response = await getModerators(tenantId, 50);
[inline-code-end]