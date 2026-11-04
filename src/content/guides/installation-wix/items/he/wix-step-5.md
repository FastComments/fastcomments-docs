כעת נגדיר את ההגדרות כך ששרשור התגובות ישתנה בהתאם לדף הנוכחי, מה שיאפשר למשתמשים לדון בתוכן המוצג כרגע.

ללא השלבים הבאים, יהיה לכם רק שרשור תגובות גלובלי אחד לכל האתר שלכם - מה שאינו שימושי במיוחד.

#### מצב פיתוח (Dev Mode)

כדי להוסיף פונקציונליות זו, נצטרך להיכנס למה ש-Wix מכנה `Dev Mode`.

לחצו על האפשרות `Dev Mode` בראש המסך.

<div class="screenshot white-bg">
    <div class="title">הפעלת מצב פיתוח</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="הפעלת מצב פיתוח" />
</div>

#### הגדרת מזהה האלמנט

נוסיף קוד מותאם אישית כדי להשיג זאת, אך קודם עלינו לתת לאלמנט ההטמעה החדש מזהה (ID) שבאמצעותו נוכל להתייחס אליו.

נקרא לו `fastcomments`.

לחצו על אלמנט ההטמעה החדש שהוספנו, ובמצב פיתוח בפינה הימנית התחתונה תראו שדה מזהה עם ערך כמו `html1`:

<div class="screenshot white-bg">
    <div class="title">שדה המזהה</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="שדה המזהה" />
</div>

שנו זאת ל-`fastcomments` והקישו Enter:

<div class="screenshot white-bg">
    <div class="title">הגדרת המזהה</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="הגדרת המזהה" />
</div>

כעת נוכל להוסיף את הקוד המותאם אישית שמציין לאזור התגובות באיזה דף אנו צופים.

בתחתית המסך תראו עורך קוד כזה:

<div class="screenshot white-bg">
    <div class="title">פתיחת העורך</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="הגדרת המזהה" />
</div>

העתיקו את הקוד הבא והדביקו אותו שם:

[inline-code-attrs-start title = 'קטע קוד לניווט תגובות ב-Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">הוספת קוד הניווט</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="הוספת קוד הניווט" />
</div>