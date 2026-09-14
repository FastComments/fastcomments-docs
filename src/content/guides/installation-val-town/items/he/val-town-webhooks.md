A val הוא מקלט ווב‑הוק טבעי: יש לו URL יציב, הוא יכול לאמת חתימה, ויש לו SQLite ואחסון בלוב מובנים.

FastComments חותמת `${timestamp}.${body}` עם סוד ה‑API של החשבון שלך ושולחת שני כותרות:

[inline-code-attrs-start title = 'כותרות ווב-הוק'; type='text' inline-code-attrs-end]
[inline-code-start]
X-FastComments-Timestamp: 1789004710        unix seconds, not milliseconds
X-FastComments-Signature: sha256=<hex>
[inline-code-end]

השיטה נושאת את האירוע: **PUT** עבור תגובה שנוצרה או עודכנה, **DELETE** עבור תגובה שנמחקה.

[inline-code-attrs-start title = 'אימות מסירה'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { createHmac, timingSafeEqual } from "node:crypto";

async function receive(c) {
  // הבתים המדויקים שהגיעו. אל תשתמש ב‑c.req.json() ואל תסיריאליז מחדש.
  const rawBody = await c.req.raw.text();
  const timestamp = c.req.raw.headers.get("X-FastComments-Timestamp");
  const signature = c.req.raw.headers.get("X-FastComments-Signature");

  if (!timestamp || !signature) return new Response("Missing headers", { status: 400 });

  // דחה משלוחים ישנים כדי שהבקשה שנתפסה לא תוכל להיות משוחזרת מאוחר יותר.
  if (Math.abs(Math.floor(Date.now() / 1000) - Number(timestamp)) > 300) {
    return new Response("Timestamp outside window", { status: 400 });
  }

  const expected = "sha256=" + createHmac("sha256", Deno.env.get("FASTCOMMENTS_API_SECRET"))
    .update(`${timestamp}.${rawBody}`)
    .digest("hex");

  const a = new TextEncoder().encode(signature);
  const b = new TextEncoder().encode(expected);
  if (a.length !== b.length || !timingSafeEqual(a, b)) {
    return new Response("Signature mismatch", { status: 401 });
  }

  // ...טפל ב‑JSON.parse(rawBody)
  return Response.json({ received: true });
}

app.put("/", receive);
app.delete("/", receive);
[inline-code-end]

## שני דברים שמציקים

**אמת את הבתים הגולמיים.** ניתוח ה‑JSON וסיריאליזציה מחדש משנים את סדר המפתחות והרווחים, ולכן ההאש שונה וכל משלוח נכשל ללא סיבה ברורה. זהו הסיבה הרגילה שמקלט ווב‑הוק "פשוט לא עובד".

**השווה בזמן קבוע.** השוואה פשוטה `===` על החתימה חושפת כמה בתים תואמים, וזה מספיק כדי לזייף בת אחד בכל פעם.

## טיפול באירועים

ענה במהירות. FastComments מנסה מחדש על תגובה שאינה 2xx, וקצה שממשיך להיכשל מושבת בסופו של דבר באופן אוטומטי, ולכן יש לבצע עבודה אמיתית אחרי שליחת התגובה ולא באופן מקומי.

הפוך את הפעולה הזאת לאידמפוטנטית על מזהה ההערה. ניסיון חוזר נחתם מחדש עם חותמת זמן חדשה, וה‑comment id עצמו מגיע שוב בעריכה ובמחיקה, ולכן אין דבר יציב שניתן להשתמש בו לדדופלקציה.

---