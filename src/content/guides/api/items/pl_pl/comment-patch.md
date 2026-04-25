[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia aktualizację pojedynczego komentarza.

Notes:

- Ta metoda API może zaktualizować widżet komentarzy "na żywo", jeśli chcesz (to zwiększa podstawowy `creditsCost` z `1` do `2`).
  - To pozwala na przeprowadzanie migracji komentarzy między stronami "na żywo" (zmiana `urlId`).
  - Migracje kosztują dodatkowe `2` kredyty, ponieważ strony są przeliczane wstępnie i jest to intensywne obliczeniowo.
- W przeciwieństwie do API tworzenia, to API NIE będzie automatycznie tworzyć obiektów użytkowników w naszym systemie, jeśli podany zostanie e-mail.
- Komentarze zaktualizowane przez to API nadal mogą być sprawdzane pod kątem spamu, jeśli chcesz.
- Konfiguracje takie jak maksymalna długość komentarza, jeśli są skonfigurowane na stronie administracyjnej Customization Rule, będą miały zastosowanie tutaj.
- Aby pozwolić użytkownikom na aktualizację tekstu komentarza, możesz po prostu określić `comment` w ciele żądania. Wygenerujemy wynikowy `commentHTML`.
  - Jeśli zdefiniujesz zarówno `comment`, jak i `commentHTML`, nie wygenerujemy automatycznie HTML.
  - Jeśli użytkownik doda wzmianki lub hashtagi w nowym tekście, będą one przetwarzane jak w API `POST`.
- Podczas aktualizacji `commenterEmail` w komentarzu najlepiej jest również podać `userId`. W przeciwnym razie musisz upewnić się, że użytkownik z tym adresem e-mail należy do Twojego tenanta, w przeciwnym razie żądanie zakończy się niepowodzeniem.  
- Jeśli docelowy komentarz jest zablokowany (`isLocked: true`), żądanie zostanie odrzucone z `code: 'locked'`. Najpierw odblokuj komentarz, zaktualizuj go, a następnie ponownie zablokuj, jeśli chcesz.


[inline-code-attrs-start title = 'Minimalny przykład żądania cURL PATCH komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** Użytkownik wykonujący aktualizację. W razie potrzeby można go użyć do sprawdzenia, czy może edytować komentarz.  **/
    contextUserId?: string
	/** Czy powinniśmy sprawdzić, czy nowy komentarz wygląda na spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Czy komentarz powinien być widoczny "na żywo" dla użytkowników przeglądających wystąpienia widżetu komentarzy z tym samym urlId. UWAGA: Podwaja koszt kredytów z 1 do 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PATCH komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]