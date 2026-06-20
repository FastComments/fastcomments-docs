## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| page | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| skip | integer | query | Nie |  |
| asTree | boolean | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| urlId | string | query | Nie |  |
| userId | string | query | Nie |  |
| anonUserId | string | query | Nie |  |
| contextUserId | string | query | Nie |  |
| hashTag | string | query | Nie |  |
| parentId | string | query | Nie |  |
| direction | string | query | Nie |  |
| fromDate | integer | query | Nie |  |
| toDate | integer | query | Nie |  |

## Odpowiedź

Zwraca: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (opcjonalne)
    limit = 56 # int |  (opcjonalne)
    skip = 56 # int |  (opcjonalne)
    as_tree = True # bool |  (opcjonalne)
    skip_children = 56 # int |  (opcjonalne)
    limit_children = 56 # int |  (opcjonalne)
    max_tree_depth = 56 # int |  (opcjonalne)
    url_id = 'url_id_example' # str |  (opcjonalne)
    user_id = 'user_id_example' # str |  (opcjonalne)
    anon_user_id = 'anon_user_id_example' # str |  (opcjonalne)
    context_user_id = 'context_user_id_example' # str |  (opcjonalne)
    hash_tag = 'hash_tag_example' # str |  (opcjonalne)
    parent_id = 'parent_id_example' # str |  (opcjonalne)
    direction = client.SortDirections() # SortDirections |  (opcjonalne)
    from_date = 56 # int |  (opcjonalne)
    to_date = 56 # int |  (opcjonalne)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]

---