Next, let's set things up so that the comment thread changes based on the current page, allowing users to discuss the currently displayed content.

כעת נגדיר את הדברים כך ששרשור ההערות ישתנה בהתאם לדף הנוכחי, מה שיאפשר למשתמשים לדון בתוכן המוצג כעת.

Without the following steps, you will only have one global comment thread for your entire site - which is not very useful.

ללא השלבים הבאים, יהיה לכם שרשור הערות גלובלי אחד לכל האתר — וזה לא מאוד שימושי.

#### מצב פיתוח

To add this functionality, we'll have to go into what Wix calls `Dev Mode`.

כדי להוסיף פונקציונליות זו, נצטרך להיכנס למה ש-Wix קוראת `Dev Mode`.

Click the `Dev Mode` option at the top of the screen.

לחצו על האפשרות `Dev Mode` בחלק העליון של המסך.

<div class="screenshot white-bg">
    <div class="title">הפעלת מצב פיתוח</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="הפעלת מצב פיתוח" />
</div>

#### הגדר את מזהה האלמנט

We're going to add custom code to accomplish this, but first we need to give the new embed element an ID to refer to it by.

נוסיף קוד מותאם כדי לבצע זאת, אבל קודם עלינו לתת לרכיב ההטמעה החדש מזהה שנוכל להתייחס אליו.

Let's call it `fastcomments`.

נקרא לו `fastcomments`.

Click the new embed element we added, and in dev mode in the bottom right you should see an ID field with a value like `html1`:

לחצו על רכיב ההטמעה החדש שהוספנו, ובמצב הפיתוח בפינה הימנית התחתונה אמור להופיע שדה ID עם ערך כמו `html1`:

<div class="screenshot white-bg">
    <div class="title">שדה ה-ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="שדה ה-ID" />
</div>

Change this to `fastcomments` and hit enter:

שנו זאת ל-`fastcomments` ולחצו Enter:

<div class="screenshot white-bg">
    <div class="title">הגדר את ה-ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="הגדר את ה-ID" />
</div>

Now we can add our custom code that tells the comment area what page we are viewing.

כעת נוכל להוסיף את הקוד המותאם שלנו שמודיע לאזור ההערות איזה דף מוצג כעת.

At the bottom of the screen you should see a code editor like this:

בתחתית המסך אמור להופיע עורך קוד כזה:

<div class="screenshot white-bg">
    <div class="title">פתח את העורך</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="פתח את העורך" />
</div>

Copy the following code and paste it in there:

העתיקו את הקוד הבא והדביקו אותו שם:

[inline-code-attrs-start title = 'קטע ניווט לתגובות של Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">הוסף את קוד הניווט</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="הוסף את קוד הניווט" />
</div>

---