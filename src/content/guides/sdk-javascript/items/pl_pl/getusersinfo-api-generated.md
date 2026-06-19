Masowe informacje o użytkownikach dla najemcy. Na podstawie userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widget komentarzy do wzbogacania użytkowników, którzy właśnie pojawili się w wyniku zdarzenia obecności.
Brak kontekstu strony: prywatność jest egzekwowana jednolicie (profile prywatne są maskowane).

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
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo wymaga tylko tenantId i ids; parametry opcjonalne nie mają tu zastosowania.
[inline-code-end]

---