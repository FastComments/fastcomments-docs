| תג | תיאור |
| --- | --- |
| `fastcomments` | מערכת תגובות חיה עם תגובות משנה, הצבעה, ניהול ועדכונים בזמן אמת |
| `fastcomments_comment_count` | ספירת תגובות עבור העמוד הנוכחי |
| `fastcomments_comment_count_bulk` | ספירות תגובות עבור דפים מרובים בעמוד רשימה/אינדקס אחד |
| `fastcomments_live_chat` | ווידג'ט צ'אט בזמן אמת |
| `fastcomments_collab_chat` | תגובות שיתופיות בתוך הטקסט (הערות טקסט) |
| `fastcomments_image_chat` | הערות על תמונות |
| `fastcomments_recent_comments` | תגובות אחרונות ברחבי האתר |
| `fastcomments_recent_discussions` | שרשורי דיון פעילים לאחרונה |
| `fastcomments_reviews_summary` | סיכום ביקורות בדירוג כוכבים |
| `fastcomments_top_pages` | הדפים הנדונים ביותר |
| `fastcomments_user_activity_feed` | פיד פעילות לכל משתמש |

### דוגמאות

```liquid
{% raw %}{# ספירת תגובות. הווידג'ט מציג את התווית שלו, לדוגמה "0 תגובות" #}
{% fastcomments_comment_count %}

{# צ'אט בזמן אמת #}
{% fastcomments_live_chat %}

{# צ'אט שיתופי. כוונו אותו לאלמנט תוכן באמצעות בוחר CSS #}
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>
{% fastcomments_collab_chat target="#post-body" %}

{# צ'אט תמונה. כוונו אותו לאלמנט תמונה באמצעות בוחר CSS #}
<img id="hero" src="/hero.jpg" alt="Hero image">
{% fastcomments_image_chat target="#hero" %}

{# סיכום ביקורות #}
{% fastcomments_reviews_summary %}

{# פיד פעילות משתמש. דורש מזהה משתמש #}
{% fastcomments_user_activity_feed user_id="demo:demo-user" %}

{# ספירות תגובות מרובות עבור אינדקס בלוג #}
{% for post in site.posts %}
  <a href="\{{ post.url }}">\{{ post.title }}</a>
  <span class="fast-comments-count" data-fast-comments-url-id="\{{ post.url }}"></span>
{% endfor %}
{% fastcomments_comment_count_bulk %}{% endraw %}
```