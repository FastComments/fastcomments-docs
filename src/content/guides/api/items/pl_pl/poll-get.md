[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Odczytuje ankietę dołączoną do komentarza, wraz z aktualnymi liczbami głosów.

Ankiety są również zwracane w samym komentarzu przez API komentarzy, więc użyj tego, gdy chcesz tylko wyniki, a nie cały komentarz.

[inline-code-attrs-start title = 'Przykład cURL Pobierania Ankiety'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Żądania Pobierania Ankiety'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Odpowiedzi Pobierania Ankiety'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Komentarz, który nie ma ankiety, komentarz, który został usunięty, oraz identyfikator komentarza, który nie istnieje, wszystkie odpowiadają w ten sam sposób, z `poll-not-found`.

---