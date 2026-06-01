---
| שורת קוד | תיאור |
| --- | --- |
| `fastcomments` | תגובות מקוננות עם תשובות, הצבעות ואזכורים ב-@ |
| `fastcomments-comment-count` | מספר תגובות לעמוד יחיד |
| `fastcomments-comment-count-bulk` | מספרי תגובות עבור עמודים רבים בבקשה אחת (ראה [ספירת תגובות מרוכזת](#bulk-comment-counts-readme-generated)) |
| `fastcomments-live-chat` | וידג'ט צ'אט חי |
| `fastcomments-collab-chat` | הערות שיתופיות בתוך הטקסט (דורש `target`) |
| `fastcomments-image-chat` | הערות על תמונה (דורש `target`) |
| `fastcomments-recent-comments` | תגובות אחרונות ברחבי האתר |
| `fastcomments-recent-discussions` | שרשורי דיון שהיו פעילים לאחרונה |
| `fastcomments-reviews-summary` | סיכום ביקורות עם דירוג בכוכבים |
| `fastcomments-top-pages` | הדפים המדוברים ביותר |
| `fastcomments-user-activity-feed` | פיד פעילות לכל משתמש (דורש `userId`) |

### דוגמאות

ספירת תגובות משולבת בתוך הטקסט:

```text
This page has \{{< fastcomments-comment-count >}} comments.
```

צ'אט חי:

```text
\{{< fastcomments-live-chat >}}
```

צ'אט שיתופי, המכוון לאלמנט תוכן באמצעות סלקטור CSS:

```text
<article id="post-body">
  <p>Highlight me to leave a comment.</p>
</article>

\{{< fastcomments-collab-chat target="#post-body" >}}
```

צ'אט לתמונות, המכוון לאלמנט תמונה באמצעות סלקטור CSS:

```text
<img id="hero" src="/hero.jpg" alt="Hero image" />

\{{< fastcomments-image-chat target="#hero" >}}
```

סיכום ביקורות:

```text
\{{< fastcomments-reviews-summary >}}
```

פיד פעילות משתמש:

```text
\{{< fastcomments-user-activity-feed userId="demo:demo-user" >}}
```
---