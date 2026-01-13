[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia usunięcie komentarza.

Notes:

- To API może zaktualizować widżet komentarzy "na żywo", jeśli zajdzie taka potrzeba (zwiększa to `creditsCost` z `1` do `2`).
- To API usunie wszystkie komentarze podrzędne.

[inline-code-attrs-start title = 'Przykład żądania cURL usunięcia komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura żądania usunięcia komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Użytkownik wykonujący aktualizację. W razie potrzeby można użyć do sprawdzenia, czy może usunąć komentarz.  **/
    contextUserId?: string
	/** Czy komentarz powinien zostać usunięty "na żywo" dla użytkowników przeglądających instancje widżetu komentarzy z tym samym urlId. UWAGA: Podwaja koszt kredytów z 1 do 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi usunięcia komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]

---