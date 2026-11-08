हर विजेट का अपना टैग होता है। इन सभी में `**extra` कीवर्ड आर्ग्युमेंट्स स्वीकार किए जाते हैं, जो विजेट कॉन्फ़िग में जैसे हैं वैसे मर्ज कर दिए जाते हैं (camelCase कुंजियों का उपयोग करें) — नीचे दिए गए नामित आर्ग्युमेंट्स से जो कुछ भी कवर नहीं है उसके लिए।

| टैग | विजेट |
|---|---|
| `{% fastcomments %}` | टिप्पणियाँ (Comments) |
| `{% fastcomments_live_chat %}` | लाइव चैट |
| `{% fastcomments_comment_count %}` | टिप्पणी संख्या बैज |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | बल्क टिप्पणी संख्याएँ |
| `{% fastcomments_collab_chat target="#el" %}` | सहयोगात्मक (इनलाइन) चैट |
| `{% fastcomments_image_chat target="#el" %}` | इमेज एनोटेशन चैट |
| `{% fastcomments_recent_comments %}` | हाल की टिप्पणियाँ |
| `{% fastcomments_recent_discussions %}` | हाल की चर्चाएँ |
| `{% fastcomments_reviews_summary %}` | समीक्षाओं का सारांश |
| `{% fastcomments_top_pages %}` | सबसे अधिक चर्चित पृष्ठ |
| `{% fastcomments_user_activity user_id="..." %}` | किसी उपयोगकर्ता की गतिविधि फ़ीड |

नामित आर्ग्युमेंट्स विजेट की camelCase कॉन्फ़िग कुंजियों से मैप होते हैं:

| आर्ग्युमेंट | कॉन्फ़िग कुंजी | टैग |
|---|---|---|
| `url_id` | `urlId` | comments, live chat, comment count, collab/image chat, recent comments, reviews summary |
| `url` | `url` | comments, live chat, collab/image chat |
| `readonly` | `readonly` | comments, live chat, collab/image chat |
| `locale` | `locale` | comments, live chat, collab/image chat, user activity |
| `has_dark_background` | `hasDarkBackground` | सभी |
| `default_sort_direction` | `defaultSortDirection` | comments, live chat, collab/image chat |
| `number_only` | `numberOnly` | comment count |
| `is_live` | `isLive` | comment count |
| `count` | `count` | recent comments, recent discussions |
| `target` | (querySelector, भेजा नहीं जाता) | collab chat, image chat |
| `chat_square_percentage` | `chatSquarePercentage` | image chat |
| `user_id` | `userId` | user activity |

उदाहरण:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Collab chat पेज पर मौजूद किसी एलिमेंट से जुड़ता है #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# बल्क काउंट्स: मार्कर रखें, फिर एक बल्क लोडर उन सभी को भर देता है #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```