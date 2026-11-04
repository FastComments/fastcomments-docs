Zbiorcze informacje o użytkownikach dla tenanta. Na podstawie userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widżet komentarzy do wzbogacenia użytkowników, którzy właśnie pojawili się przez zdarzenie presence.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (prywatne profile są maskowane).

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| ids | string | Tak |  |

## Odpowiedź

Zwraca: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]