Zbiorcze informacje o użytkownikach dla najemcy. Dla podanych userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widget komentarzy do wzbogacenia użytkowników, którzy właśnie pojawili się za pomocą zdarzenia obecności.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (profile prywatne są zamaskowane).

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| ids | string | Tak |  |

## Odpowiedź

Zwraca: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // opcjonalne; jeśli undefined, domyślnie przecinek
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]