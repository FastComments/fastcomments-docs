כל אפשרויות הווידג'ט של FastComments מוגדרות תחת `[params.fastcomments]` ב-`hugo.toml`, וניתן לעקוף אותן לכל דף בנפרד ב-front matter תחת `[fastcomments]`. קדימות, מהנמוכה לגבוהה: פרמטרי האתר, front matter של הדף, פרמטרי ה-shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

אם לא סופקו אף אחד מ-`url` או `urlId`, הערך של `url` יוגדר כברירת מחדל לקישור הקבוע (permalink) של העמוד כדי ששרשורי התגובות יישארו קשורים ל-URL יציב.

### מיקום אחסון נתונים (EU)

לקוחות ב-EU מגדירים `region = "eu"` כדי לנתב את הווידג'ט אל `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### הערה לגבי אותיות המפתח (key casing)

Hugo ממיר את כל המפתחות ל-אותיות קטנות בקובץ `hugo.toml` וב-front matter, אבל הווידג'טים של FastComments דורשים מפתחות ב-camelCase (`tenantId`, `hasDarkBackground`). רכיב זה משחזר אוטומטית את הרישיות הנכונה עבור כל אפשרות ידועה ברמת השורש, לכן כתבו את האפשרויות בצורת ה-camelCase הרגילה. מפתחות שממוקמים בתוך ערך אובייקט (למשל המפתחות של מפת `translations`, או שדות של `pageReactConfig`) אינם משוחזרים. הגדר אותן דרך ממשק ההתאמה האישית בלוח הבקרה של FastComments במקום זאת.