Every widget has its own tag. All of them accept `**extra` keyword arguments,  
which are merged into the widget config as‑is (use camelCase keys) for anything  
not covered by the named arguments below.

| תג | וידג'ט |
|---|---|
| `{% fastcomments %}` | תגובות |
| `{% fastcomments_live_chat %}` | צ’אט חי |
| `{% fastcomments_comment_count %}` | תג צבע ספירת תגובות |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | ספירות תגובות גורפות |
| `{% fastcomments_collab_chat target="#el" %}` | צ’אט שיתופי (מוטמע) |
| `{% fastcomments_image_chat target="#el" %}` | צ’אט הערת תמונה |
| `{% fastcomments_recent_comments %}` | תגובות אחרונות |
| `{% fastcomments_recent_discussions %}` | דיונים אחרונים |
| `{% fastcomments_reviews_summary %}` | סיכום ביקורות |
| `{% fastcomments_top_pages %}` | דפים עם הכי הרבה דיונים |
| `{% fastcomments_user_activity user_id="..." %}` | פיד פעילות של משתמש |

Named arguments map to the widget's camelCase config keys:

| ארגומנט | מפתח תצורה | תגיות |
|---|---|---|
| `url_id` | `urlId` | comments, live chat, comment count, collab/image chat, recent comments, reviews summary |
| `url` | `url` | comments, live chat, collab/image chat |
| `readonly` | `readonly` | comments, live chat, collab/image chat |
| `locale` | `locale` | comments, live chat, collab/image chat, user activity |
| `has_dark_background` | `hasDarkBackground` | all |
| `default_sort_direction` | `defaultSortDirection` | comments, live chat, collab/image chat |
| `number_only` | `numberOnly` | comment count |
| `is_live` | `isLive` | comment count |
| `count` | `count` | recent comments, recent discussions |
| `target` | (querySelector, not sent) | collab chat, image chat |
| `chat_square_percentage` | `chatSquarePercentage` | image chat |
| `user_id` | `userId` | user activity |

דוגמאות:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Collab chat מצורף לאלמנט קיים בדף #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# ספירות מרובות: מיקום מסמנים, ואז טוען קבוצתי אחד ממלא את כולם #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```