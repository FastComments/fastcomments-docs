אם התקנת את FastComments מחנות האפליקציות של Shopify, הדומיין של החנות שלך נוסף אוטומטית לדומיינים המורשים של ה-tenant ולא אמורה להופיע שגיאת דומיין. דף זה חל אם ביצעת את נתיב ההתקנה הידני, או אם החזית החנות שלך מוגשת על דומיין מותאם אישית שלא נרשם ב-Shopify בזמן התקנת האפליקציה.

ייתכן שתקבל שגיאת הרשאה שנראית כך:

<div class="screenshot white-bg">
    <div class="title">הגדרת דומיין חסרה</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-webflow-step-5.png" alt="הגדרת דומיין חסרה" />
</div>

זה אומר ש-FastComments לא מזהה את הדומיין שעליו הווידג'ט נטען כמאושר עבור ה-tenant שלך.

כדי לתקן זאת, הוסף את הדומיין לחשבון FastComments שלך: [Configure Domains](https://fastcomments.com/auth/my-account/configure-domains).