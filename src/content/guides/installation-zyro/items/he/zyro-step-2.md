---
עכשיו נוסיף את קוד הווידג'ט שלנו.

העתיקו את הקוד למטה. עליכם לוודא שאתם מחוברים ל-[fastcomments.com](https://fastcomments.com) 
ולטעון מחדש את הדף אם לא, כך שהקוד ימולא מראש עם פרטי החשבון שלכם; אחרת הוא יציג את קוד הדמו.

עכשיו נעתיק את הקוד:

[inline-code-attrs-start title = 'קוד תגובות Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

עכשיו נחזור לבונה האתר שלנו ונלחץ על `Enter code`:

<div class="screenshot white-bg">
    <div class="title">הזן קוד</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="הזן קוד" />
</div>

### הערה!

חשוב להשתמש בקוד שלמעלה ולא בקטעי קוד ממסמכי תיעוד אחרים, כיוון שקטע הקוד הזה נוצר במיוחד
עבור Zyro.

כעת אמור להיות לכם משהו כזה, שנראה ריק. זה צפוי. העבירו את העכבר מעל האזור
שבו אמור להופיע הווידג'ט:

<div class="screenshot white-bg">
    <div class="title">הווידג'ט הוסף</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="הווידג'ט הוסף" />
</div>

כעת גררו את הווידג'ט לגודל הרצוי — תראו אותו מופיע:

<div class="screenshot white-bg">
    <div class="title">שנה את הגודל</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="שנה את הגודל" />
</div>

...ועכשיו צפו בתצוגה מקדימה ושמרו!

---