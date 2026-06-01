כדי להציג ספירת תגובות לצד מספר דפים בבת אחת (למשל אינדקס בלוג או רשימת מדור), השתמשו בווידג'ט ספירת-במקבץ. הוא מאחזר את כל הספירות שבדף בבקשה אחת. יש שני חלקים: סמן לצד כל פריט, וקריאת אתחול אחת אחרי הרשימה.

בתבנית רשימה (`layouts/_default/list.html`):

```go-html-template
<ul>
  \{{ range .Pages }}
    <li>
      <a href="\{{ .RelPermalink }}">\{{ .Title }}</a>
      \{{ partial "fastcomments/count-marker.html" . }}
    </li>
  \{{ end }}
</ul>
\{{ partial "fastcomments/bulk-count.html" (dict "page" .) }}
```

`count-marker.html` מציג `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, תוך שימוש באותו מזהה שהווידג'ט תגובות משתמש עבור אותו דף (ה-`urlId` שלו, או הקישור הקבוע שלו כשה-`urlId` לא מוגדר), כך שהספירות מתיישרות עם שרשורי התגובות האמיתיים. `bulk-count.html` מבצע את הבקשה היחידה שממלאת אותן.

אם אתם כותבים את הסמנים בעצמכם (למשל ב-Markdown של הדף), השתמשו ב-shortcode כדי להוציא במקום זאת את קריאת האתחול:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```