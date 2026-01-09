[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

To API używa paginacji, realizowanej przez parametr zapytania `skip`. Moderatorzy są zwracani stronami po `100`, posortowani według `createdAt` i `id`.

Koszt zależy od liczby zwróconych moderatorów — `1 credit per 10` zwróconych moderatorów.

[inline-code-attrs-start title = 'Przykład cURL dla Moderatora'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Liczba moderatorów do pominięcia w paginacji. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]

---