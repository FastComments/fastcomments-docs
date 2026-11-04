לכל ווידג'ט יש תג משלו. כולם מקבלים ארגומנטים של מילות מפתח `**extra`,
אשר ממוזגים לתצורת הווידג'ט כפי שהם (השתמשו במפתחות camelCase) עבור כל דבר
שאינו מכוסה על ידי הארגומנטים המכוננים למטה.

| תג | ווידג'ט |
|---|---|
| `{% fastcomments %}` | תגובות |
| `{% fastcomments_live_chat %}` | צ'אט חי |
| `{% fastcomments_comment_count %}` | תג ספירת תגובות |
| `{% fastcomments_comment_count_bulk %}` + `{% fastcomments_count_marker %}` | ספירות תגובות מרובות |
| `{% fastcomments_collab_chat target="#el" %}` | צ'אט שיתופי (מוטמע) |
| `{% fastcomments_image_chat target="#el" %}` | צ'אט הערות על תמונות |
| `{% fastcomments_recent_comments %}` | תגובות אחרונות |
| `{% fastcomments_recent_discussions %}` | דיונים אחרונים |
| `{% fastcomments_reviews_summary %}` | סיכום ביקורות |
| `{% fastcomments_top_pages %}` | הדפים המדוברים ביותר |
| `{% fastcomments_user_activity user_id="..." %}` | פיד פעילות של משתמש |

הארגומנטים המכוננים ממופים למפתחות התצורה ב-camelCase של הווידג'ט:

| ארגומנט | מפתח תצורה | תגים |
|---|---|---|
| `url_id` | `urlId` | תגובות, צ'אט חי, ספירת תגובות, צ'אט שיתופי/תמונות, תגובות אחרונות, סיכום ביקורות |
| `url` | `url` | תגובות, צ'אט חי, צ'אט שיתופי/תמונות |
| `readonly` | `readonly` | תגובות, צ'אט חי, צ'אט שיתופי/תמונות |
| `locale` | `locale` | תגובות, צ'אט חי, צ'אט שיתופי/תמונות, פעילות משתמש |
| `has_dark_background` | `hasDarkBackground` | הכול |
| `default_sort_direction` | `defaultSortDirection` | תגובות, צ'אט חי, צ'אט שיתופי/תמונות |
| `number_only` | `numberOnly` | ספירת תגובות |
| `is_live` | `isLive` | ספירת תגובות |
| `count` | `count` | תגובות אחרונות, דיונים אחרונים |
| `target` | (querySelector, לא נשלח) | צ'אט שיתופי, צ'אט תמונות |
| `chat_square_percentage` | `chatSquarePercentage` | צ'אט תמונות |
| `user_id` | `userId` | פעילות משתמש |

דוגמאות:

```django
{% load fastcomments %}

{% fastcomments url_id="my-page" locale="en_us" default_sort_direction="MR" %}

{% fastcomments_live_chat url_id="room-1" %}

Comments: {% fastcomments_comment_count url_id="my-page" number_only=True %}

{# Collab chat attaches to an existing element on the page #}
<article id="post-body">...</article>
{% fastcomments_collab_chat target="#post-body" %}

{# Bulk counts: place markers, then one bulk loader fills them all in #}
{% for post in posts %}
    <a href="\{{ post.url }}">\{{ post.title }}</a>
    {% fastcomments_count_marker url_id=post.url_id %}
{% endfor %}
{% fastcomments_comment_count_bulk %}
```