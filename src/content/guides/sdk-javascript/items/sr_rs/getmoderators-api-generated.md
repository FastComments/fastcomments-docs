## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = "acme-tenant-98765";
  const moderatorsResponse: GetModeratorsResponse = await getModerators(tenantId);
  const skip: number = 25;
  const pagedResponse: GetModeratorsResponse = await getModerators(tenantId, skip);
  console.log(moderatorsResponse, pagedResponse);
}
run();
[inline-code-end]

---