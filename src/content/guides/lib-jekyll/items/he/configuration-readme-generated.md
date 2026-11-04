הקונפיגורציה מגיעה משלושה מקורות. מקורות מאוחרים גוברים:

1. **ברירות מחדל גלובליות** ב-`_config.yml` תחת המפתח `fastcomments:`.
2. **הקשר העמוד**, שנגזר באופן אוטומטי עבור ווידג׳טים בהיקף עמוד (ראו להלן).
3. **מאפייני תגית** שנכתבים על התגית עצמה.

כלומר, `url_id` על התגית דורס את הערך הנגזר מהעמוד, שדורס כל ברירת מחדל גלובלית.

### תחביר מאפיינים

מאפיינים הם זוגות `key=value` בפורמט `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- ערכים **מצוטטים** (`"..."` או `'...'`) הם מחרוזות ליטרליות.
- `true`/`false` **ללא מירכאות** הופכים לערכים בוליאניים, ומספרים הופכים למספרים.
- כל דבר אחר **ללא מירכאות** מפוענח כמשתנה Liquid מהקשר העמוד, למשל
  `url_id=page.slug`. (Liquid אינו מרחיב `{% raw %}\{{ ... }}{% endraw %}` בתוך
  מאפייני תגית, ולכן יש להשתמש בצורה החשופה `page.slug` ולא ב-`"{% raw %}\{{ page.slug }}{% endraw %}"`.)

מפתחות מאפיינים וקונפיגורציה בפורמט snake_case ממופים באופן אוטומטי למפתחות camelCase ש-FastComments
מצפה להם (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground` וכן הלאה). כל אפשרות אחרת מתוך
[הגדרות הווידג׳ט](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
מועברת ישירות באותו אופן.

### ערכים הנגזרים מהעמוד

עבור הווידג׳טים בהיקף עמוד (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`) הערכים הללו ממולאים באופן אוטומטי מהעמוד הנוכחי,
אלא אם הגדרתם אותם בעצמכם:

- `url_id` ← `page.url` (מזהה יציב שאינו תלוי בדומיין המבקר)
- `url` ← `site.url` + `page.url` (רק כאשר `url` מוגדר ב-`_config.yml`)
- `page_title` ← `page.title`

ווידג׳טים ברמת האתר (תגובות/דיונים אחרונים, עמודים מובילים, סיכום ביקורות, פיד פעילות משתמשים,
ספירה מרובה) אינם קשורים לעמוד ואינם גוזרים את הערכים הללו.

### אחסון נתונים באיחוד האירופי

לקוחות באיחוד האירופי מוסיפים `region: eu`, באופן גלובלי:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

או לכל תגית: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. הווידג׳טים נטענים אז מה-CDN של האיחוד האירופי.