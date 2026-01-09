[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность обновить одну `Page`. Соответствующие комментарии будут обновлены.

[inline-code-attrs-start title = 'Пример cURL-запроса обновления страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса обновления страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа обновления страницы'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Указывается при ошибке. **/
    reason?: string
    user?: Page; // При успешном выполнении возвращаем полностью обновлённую страницу.
}
[inline-code-end]

#### Примечание

Некоторые параметры в объекте Page обновляются автоматически. Это — счетчики и атрибуты title. Счетчики нельзя изменить через API, так как они являются вычисляемыми значениями. `title` страницы можно задать через API, но он будет перезаписан, если виджет комментариев используется на странице с тем же `urlId` и другим заголовком страницы.

---