## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| textSearch | string | Не |  |
| byIPFromComment | string | Не |  |
| filter | string | Не |  |
| searchFilters | string | Не |  |
| demo | boolean | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Пример

[inline-code-attrs-start title = 'getCount Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const countResult: ModerationAPICountCommentsResponse = await getCount(
    "reported harassment",
    "203.0.113.7",
    "status:pending",
    undefined,
    false,
    "sso_user_789.jwt.token"
  );
  console.log(countResult);
})();
[inline-code-end]

---