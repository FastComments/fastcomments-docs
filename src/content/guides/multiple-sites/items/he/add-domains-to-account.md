FastComments מאמתת בקשות לחשבונך כדי לוודא שהן מגיעות מהאתר שלך. לכן
אנו צריכים לדעת באיזה אתר, או באילו אתרים, ברצונך להתקין את FastComments.

FastComments תומכת באימות על ידי שם דומיין, כמו גם תתי-דומיינים.

ניקח את האתר `https://example.com`. במקרה זה, "`example.com`" הוא שם הדומיין. `example.com` תומך גם ב-`example.com`, וגם ב-`www.example.com`. ננקוב ב-"www" כ" תת-דומיין".

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

אם השתמשת בפלטפורמת בלוגים, וניתן לך תת-דומיין, תרצה
להוסיף את **הדומיין המלא כולל תת-הדומיין** לחשבונך, לדוגמה: `cats.blogger.com`.

We can add domains to our account by visiting the `My Domains` page and clicking `Add a Domain` at the bottom:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

במהלך תקופת הניסיון, **דומיינים מתווספים אוטומטית לחשבונך** כאשר בקשות מגיעות מהדומיינים הנ"ל. עם זאת,
לאחר תקופה זו הם חייבים להתווסף במפורש מטעמי אבטחה. עליך לקבל הודעת דואר אלקטרוני כאשר התנהגות אוטומטית זו מתרחשת.

אתה **לא** חייב להוסיף את `localhost` לפיתוח מקומי - הוא מותר כברירת מחדל.

#### דרך ה-API

Domains can also be added and configured [via the DomainConfigs API](/guide-api.html#domain-config-structure).