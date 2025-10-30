There are a few endpoints to get the counts, depending on what you want and if you want to get them from a browser, server, or using the
API SDK.

## Public Comment Counts

You can get the public comment counts using the widgets above or using the APIs they use. These APIs remain unchanged since 2019 and
will never change.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

This will return a structure like:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

The `postfix` property is always included. 

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

This will return a structure like:

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

The `counts` object is only populated for pages that have counts. The `translations` map is always present as it is used for the widget.

### Public Endpoint Behavior / Caching

The public endpoints have a 60-second caching mechanism to handle spikes in traffic. Internally this is a per-thread LRU cache in memory on the server,
so you may see counts change slightly (go up then back down temporarily) when people are leaving lots of comments.

The public endpoints always return the *total* comment count, not the root comment count. 

### Server-Side APIs / SDK

The way to get comments from your server is to call the [Pages API](/guide-api.html#page-structure) and get the page object, which contains the total comment count
and root comment count. We provide SDKs which allow you to call this API without constructing the API request manually, and provide typed return values.
