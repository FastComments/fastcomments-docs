FastComments מפעילה שרת Model Context Protocol (MCP) מאוחסן כך שעוזרי AI ולקוחות אגנטיים יוכלו לקרוא ל-API של FastComments ישירות. כל כלי שהשרת חושף נוצר אוטומטית מהמפרט הציבורי של OpenAPI, ולכן כל מה שה-REST API יכול לעשות, לקוח MCP יכול לעשות.

ה-endpoint הוא חסר-מצב (stateless) ומבוסס streamable-HTTP. אין סשן שצריך לשמור עליו חי ואין מצב בצד השרת עבור כל לקוח.

### Endpoint

[inline-code-attrs-start title = 'MCP Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### התחברות עם OAuth

כל לקוח MCP שתומך בשרתים מרוחקים עם OAuth (Claude, ChatGPT, Claude Code, Cursor ואחרים) יכול להתחבר ל-endpoint שלמעלה ללא הגדרות בצד FastComments. הלקוח רושם את עצמו באמצעות Dynamic Client Registration או מזהה את עצמו באמצעות Client ID Metadata Document, פותח דפדפן כדי שתוכלו להתחבר ל-FastComments ולאשר גישה, ומקבל טוקן הקשור לחשבון שהייתם מחוברים אליו.

מסמכי ה-discovery נמצאים במיקומים הסטנדרטיים:

[inline-code-attrs-start title = 'Discovery'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

המשתמש שלכם צריך הרשאת API Admin בחשבון כדי לאשר חיבור. אם אתם מנהלים מספר חשבונות, עברו לחשבון הנכון בדשבורד לפני האישור.

לקוח יכול לבקש את ה-scope של `read`, את ה-scope של `write`, או את שניהם. לקוח שלא מבקש דבר מקבל את שניהם. כלים שמשנים נתונים אינם מוצעים לטוקן לקריאה בלבד.

בדשבורד יש אסיסטנט הגדרה עם קטעי קוד מוכנים להדבקה. פתחו את **Integrate -> MCP Server**, או בקרו בו ישירות:

[inline-code-attrs-start title = 'Setup Page'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

רשמו את שרת FastComments בפקודה אחת, ואז הריצו `/mcp` בתוך סשן כדי להתחבר ולראות את הכלים הזמינים:

[inline-code-attrs-start title = 'Claude Code Setup'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor ולקוחות אחרים מבוססי קובץ קונפיגורציה

הוסיפו את הבלוק הזה לקונפיגורציית שרתי ה-MCP של הלקוח שלכם (`mcp.json` עבור Cursor). הלקוח פותח דפדפן להתחברות בשימוש הראשון.

[inline-code-attrs-start title = 'MCP Client Config'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### שלילת גישה

כל חיבור מאושר מופיע תחת **Integrate -> Connected Apps** בדשבורד. שלילת חיבור מבטלת את כל הטוקנים שהאפליקציה מחזיקה. אפליקציות רושמות את עצמן כשהן מתחברות ו-FastComments אינה סוקרת אותן, ולכן שללו כל דבר שאינו מוכר לכם.

### שימוש בטוקן עם ה-REST API

טוקן הגישה שלקוח MCP מקבל הוא פרטי כניסה רגילים ל-API של FastComments. הוא עובד בכל endpoint של `/api/v1` כ-bearer token, כך שאפליקציה שהתחברה דרך MCP יכולה גם לקרוא ל-REST API ישירות:

[inline-code-attrs-start title = 'Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

ה-tenant נגזר מהטוקן. ניתן עדיין להעביר `tenantId` אך הוא חייב להתאים. בקשות `GET` דורשות את ה-scope של `read` וכל השאר דורש את ה-scope של `write`.

### התחברות עם מפתח API

לקוחות שאינם יכולים להשלים התחברות בדפדפן, כגון שרתים ללא ממשק (headless), יכולים לבצע אימות באמצעות מפתח API במקום. העבירו את `tenantId` ו-`API_KEY` כפרמטרי query, או ככותרות HTTP של `x-tenant-id` ו-`x-api-key` אם הלקוח שלכם תומך בכותרות מותאמות אישית:

[inline-code-attrs-start title = 'API Key Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

עמוד ההגדרה מייצר את ה-URL הזה עבור כל אחד ממפתחות ה-API שלכם.

### אבטחה

URL של endpoint שמכיל מפתח API הוא סוד: אל תדביקו אותו בצ'אטים ציבוריים, בצילומי מסך או בקומיטים. אם מפתח נחשף, החליפו אותו בעמוד API Keys בדשבורד שלכם. טוקני OAuth אינם נושאים סיכון כזה מכיוון שהם קשורים לאפליקציה אחת וניתן לשלול אותם מ-Connected Apps.