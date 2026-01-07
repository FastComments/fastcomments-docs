원하는 것과 브라우저, 서버 또는 API SDK에서 가져오려는지에 따라 카운트를 얻기 위한 여러 엔드포인트가 있습니다.

## 공개 댓글 수

위의 위젯을 사용하거나 해당 위젯이 사용하는 API를 사용하여 공개 댓글 수를 얻을 수 있습니다. 이러한 API는 2019년 이후 변경되지 않았으며 앞으로도 변경되지 않을 것입니다.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

다음과 같은 구조를 반환합니다:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

`postfix` 속성은 항상 포함됩니다.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

다음과 같은 구조를 반환합니다:

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

`counts` 객체는 카운트가 있는 페이지에 대해서만 채워집니다. `translations` 맵은 위젯에 사용되므로 항상 존재합니다.

### 공개 엔드포인트 동작 / 캐싱

공개 엔드포인트에는 트래픽 급증을 처리하기 위한 60초 캐싱 메커니즘이 있습니다. 내부적으로 이것은 서버 메모리의 스레드당 LRU 캐시이므로 사람들이 많은 댓글을 남길 때 카운트가 약간 변경되는 것을 볼 수 있습니다(올라갔다가 일시적으로 내려감).

공개 엔드포인트는 항상 루트 댓글 수가 아닌 *총* 댓글 수를 반환합니다.

### 서버 측 API / SDK

서버에서 댓글을 가져오는 방법은 [Pages API](/guide-api.html#page-structure)를 호출하여 총 댓글 수와 루트 댓글 수가 포함된 페이지 객체를 가져오는 것입니다. API 요청을 수동으로 구성하지 않고도 이 API를 호출할 수 있는 SDK를 제공하며 타입이 지정된 반환 값을 제공합니다.
