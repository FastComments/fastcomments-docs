אם אינכם יכולים להתקין את [אפליקציית Shopify App Store](https://apps.shopify.com/fastcomments), עדיין ניתן להוסיף את FastComments על ידי עריכת התבנית שלכם. מסלול זה שימושי כאשר ברצונכם לחבר טננט FastComments שכבר בבעלותכם, או כאשר אתם מטמיעים בחנות Shopify שבה האפליקציה אינה אפשרות.

ההתקנה דרך האפליקציה היא המסלול הנתמך עבור רוב החנויות. השתמשו במסלול זה רק אם האפליקציה אינה מתאימה.

### שלב 1: השבתת התגובות המובנות של Shopify

בממשק הניהול של Shopify, עברו אל **Blog posts > Manage blogs**, פתחו כל בלוג והגדירו **Comments are disabled** בפאנל הימני. שמרו.

פעולה זו מונעת ממערכת התגובות המובנית של Shopify להופיע לצד FastComments.

### שלב 2: פתיחת תבנית הבלוג

בממשק הניהול של Shopify:

1. עברו אל **Online Store > Themes**.
2. מתחת לתבנית הנוכחית שלכם, לחצו על **Actions > Edit code**.
3. בסייר הקבצים משמאל, פתחו את **Sections** ולחצו על `main-article.liquid`.

זו התבנית ש-Shopify מציג עבור מאמר בלוג בודד.

### שלב 3: הדבקת קטע הקוד של FastComments

גללו לקו 100 בערך של `main-article.liquid`, מיד אחרי ה-`</div>` הסוגר של גוף המאמר. הדביקו את קטע הקוד הבא:

[inline-code-attrs-start title = 'קטע קוד FastComments עבור Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

החליפו את `"demo"` ב-Tenant ID משלכם מ-[fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret). לחצו על **Save**.

### שלב 4: אישור הדומיין של החנות שלכם

פתחו פוסט בבלוג בחנות החיה שלכם. אם אתם רואים שגיאת אישור במקום ווידג'ט התגובות, FastComments צריך לדעת שהחנות שלכם מורשית להשתמש בטננט זה. ראו [שגיאות דומיין](/guide-installation-shopify.html#shopify-domain-errors).

### הוספת FastComments לדפים אחרים

אותו קטע קוד עובד בכל תבנית Liquid, כולל דפי מוצרים, דפים מותאמים אישית ודף הבית. הדביקו אותו במקום שבו ברצונכם שהתגובות יופיעו והתאימו את `urlId` אם ברצונכם מזהה קבוע לכל דף (למשל, `urlId: "{{ product.id }}"` בתבנית מוצר).