## Параметры

| Имя | Тип | Required | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Ответ

Возвращает: [`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // необязательное смещение пагинации
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // Пример без пагинации:
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]

---