## Parameters

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`GetModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-001";
  const id: string = "mod-12345";

  const result: GetModeratorResponse1 = await getModerator(tenantId, id);

  const moderatorName: string | undefined = result.moderator?.name;
  console.log(moderatorName);
})();
[inline-code-end]