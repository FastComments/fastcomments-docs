[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia aktualizację pojedynczego komentarza.

Notes:

- To API może zaktualizować widżet komentarzy „na żywo”, jeśli chcesz (to zwiększa podstawowy `creditsCost` z `1` do `2`).
  - To może sprawić, że migracje komentarzy między stronami będą „na żywo” (zmieniając `urlId`).
  - Migracje kosztują dodatkowe `2` kredyty, ponieważ strony są wstępnie przeliczane i jest to proces intensywny dla CPU.
- W przeciwieństwie do API tworzenia, to API NIE utworzy automatycznie obiektów użytkownika w naszym systemie, jeśli zostanie podany adres e-mail.
- Komentarze zaktualizowane przez to API nadal mogą być sprawdzane pod kątem spamu, jeśli chcesz.
- Ustawienia takie jak maksymalna długość komentarza, jeśli skonfigurowane na stronie administracyjnej Customization Rule, będą miały tutaj zastosowanie.
- Aby umożliwić użytkownikom aktualizację tekstu komentarza, możesz po prostu określić `comment` w ciele żądania. My wygenerujemy wynikowy `commentHTML`.
  - Jeśli zdefiniujesz zarówno `comment`, jak i `commentHTML`, nie wygenerujemy automatycznie HTML.
  - Jeśli użytkownik doda wzmianki lub hashtagi w nowym tekście, zostaną one przetworzone tak samo jak w API `POST`.
- Podczas aktualizacji `commenterEmail` w komentarzu najlepiej podać także `userId`. W przeciwnym razie musisz upewnić się, że użytkownik z tym adresem e-mail należy do Twojego tenantu, inaczej żądanie się nie powiedzie.  


[inline-code-attrs-start title = 'Minimalny przykład PATCH cURL komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania PATCH komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PATCH komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]