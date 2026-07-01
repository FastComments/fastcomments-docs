Poprzedni komentatorzy na stronie, którzy NIE są obecnie online. Sortowani według displayName.  
Użyj tego po wyczerpaniu /users/online, aby wyświetlić sekcję „Members”.  
Paginacja kursora na commenterName: serwer przegląda częściowy {tenantId, urlId, commenterName}  
indeks od afterName do przodu za pomocą $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Odpowiedź

Zwraca: [`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]

---