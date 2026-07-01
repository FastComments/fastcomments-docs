Тренутно онлајн прегледачи странице: људи чија веб‑сокет сесија је претплаћена на страницу тренутно.  
Враћа anonCount + totalCount (претплатници у просторији, укључујући анонимне прегледаче које не наводимо).

## Параметри

| Име | Тип | Неопходно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Одговор

Враћа: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Пример

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // Са опционалним параметрима страничења
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // Без опционалних параметара страничења
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]