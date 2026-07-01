## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| skip | number | Ні |  |

## Відповідь

Повертає: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'getModerators Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // опціональний зсув пагінації
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Приклад без пагінації:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]