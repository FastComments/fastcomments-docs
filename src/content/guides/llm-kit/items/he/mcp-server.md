---
FastComments מפעילה שרת Model Context Protocol (MCP) מנוהל כדי שעוזרי קוד מבוססי בינה מלאכותית ולקוחות אייג'נטיים יוכלו לקרוא ישירות ל-API של FastComments. כל כלי שהשרת MCP חושף נוצר אוטומטית מה-OpenAPI spec הציבורי, כך שכל דבר שה-REST API יכול לעשות — גם לקוח MCP יכול לעשות.

הנקודת קצה היא חסרת-מצב ומבוססת על HTTP סטרימינג. אין צורך לשמור סשן חי, אין שלב רישום לקוח, ואין מצב צד-שרת לכל לקוח.

### Endpoint

[inline-code-attrs-start title = 'נקודת קצה MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

האימות משתמש באותו מפתח API כמו ה-REST API. ניתן גם להעביר את `tenantId` והמפתח ככותרות HTTP `x-tenant-id` ו-`x-api-key` אם הלקוח שלכם תומך בכותרות מותאמות.

### Pre-filled setup

בלוח הבקרה יש עוזר הגדרה שמייצר את ה-URL וקטעי תצורה מוכנים להדבקה עבור לקוחות MCP פופולריים. גשו ללוח הבקרה של החשבון שלכם ופתחו **שילוב -> שרת MCP**, או גשו ישירות:

[inline-code-attrs-start title = 'דף ההגדרה'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

בחרו את מפתח ה-API לשימוש מהרשימה הנפתחת, ואז העתקו כל אחד מקטעי התצורה שנוצרו.

### Claude Code

רשמו את שרת FastComments באמצעות פקודה אחת:

[inline-code-attrs-start title = 'הגדרת Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

לאחר שהוא נרשם, הריצו `/mcp` בתוך סשן של Claude Code כדי לאשר את החיבור ולהציג את הכלים הזמינים.

### Claude Desktop / Cursor

הוסיפו בלוק זה לקובץ התצורה של שרתי MCP של הלקוח (`claude_desktop_config.json` עבור Claude Desktop, `mcp.json` עבור Cursor):

[inline-code-attrs-start title = 'תצורת לקוח MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### Security

מפתח ה-API מוטמע ב-URL. התייחסו ל-URL כסוד: אל תדביקו אותו בצ'אטים ציבוריים, בצילומי מסך או בקומיטים. אם מפתח נחשף — החליפו אותו בעמוד מפתחות ה-API בלוח הבקרה שלכם.

---