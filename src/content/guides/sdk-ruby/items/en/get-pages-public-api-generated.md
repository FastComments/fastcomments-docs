List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Optional case-insensitive title prefix filter. |
| sortBy | string | query | No | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical). |
| hasComments | boolean | query | No | If true, only return pages with at least one comment. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Example

[inline-code-attrs-start title = 'get_pages_public Example'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`.
  limit: 56, # Integer | 1..200, default 50
  q: 'q_example', # String | Optional case-insensitive title prefix filter.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical).
  has_comments: true # Boolean | If true, only return pages with at least one comment.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]
