עכשיו כשאנחנו בעורך התבניות, עלינו להחליט היכן נרצה להציג את ההערות או את הצ'אט החי.

בדוגמה זו נוסיף אותו ישירות מתחת לווידאו. עברו עם העכבר מעל האלמנט כדי להוסיף את הווידג'ט בסופו, ולחצו על `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">הוסף אלמנט</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="הוסף אלמנט" />
</div>

Select `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">בחרו ב־CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="בחרו ב־CUSTOM JS/HTML" />
</div>

עכשיו נפתח את עורך הקוד שבו נדביק את הקוד שלנו.

ClickFunnels is a bit confusing on this next step.

חשוב *לא לבחור* את `Code` כשמוצאים את הסמן מעל האלמנט החדש. במקום זאת, בחרו ב־`SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">בחרו ב־SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="בחרו ב־SETTINGS" />
</div>

כעת בצד ימין ניתן ללחוץ על `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">לחצו על Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="לחצו על Open Code Editor" />
</div>

יופיע ריבוע גדול שנפתח. כאן נוכל להדביק את הקוד שלנו. העתקו את הקטע הבא (השתמשו בכפתור העתק שבפינה הימנית העליונה):

[inline-code-attrs-start title = 'קטע קוד לצאט בשידור עבור ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // כמה ספקים משנים את קטע הקוד כדי שיהיה אסינכרוני
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

קטע קוד זה מיועד למוצר ה־Streaming Chat שלנו, שמתאים במיוחד לווידאוים. אם אתם רוצים במקום זאת את קטע הקוד של הווידג'ט Live Commenting, שמתאים יותר לעמודים רגילים או פוסטים בבלוג, הוא נמצא בסוף המדריך הזה.

כשנדביק את קטע הקוד בתוך החלון, זה אמור להיראות כך:

<div class="screenshot white-bg">
    <div class="title">הדביקו קוד</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="הדביקו קוד" />
</div>

כעת נותר לנו רק לסגור את התיבה:

<div class="screenshot white-bg">
    <div class="title">סגירה</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="סגירה" />
</div>

כעת תוכלו לצפות בתצוגה מקדימה של השינויים! הרגישו חופשי להזיז את הווידג'ט ולראות היכן הוא נראה לכם הכי טוב.

<div class="screenshot white-bg">
    <div class="title">תצוגה מקדימה</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="תצוגה מקדימה" />
</div>

הצלחה! אל תשכחו לבדוק במובייל!

<div class="screenshot white-bg">
    <div class="title">הצלחה!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="הצלחה!" />
</div>