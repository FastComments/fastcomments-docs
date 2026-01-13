כדי שהאינטגרציה בין Weebly ו-FastComments תעבוד היטב, עלינו להוסיף **שתי** חתיכות קוד קטנות.

הקטע הראשון מיועד להסתיר את הודעת Weebly "התגובות סגורות", והקטע השני מיועד לטעון בפועל את FastComments.

ראשית, העתק את קטע הקוד הקטן הבא:

[inline-code-attrs-start title = 'קטע קוד לכותרת של FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

ואז, באותו עמוד הגדרות מ-`Step One`, לחץ על ה-`+` שלצד `Post header code`.

<div class="screenshot white-bg">
    <div class="title">פתח קוד כותרת הפוסט</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="פתח קוד כותרת הפוסט" />
</div>

תופיע תיבת טקסט כזו:

<div class="screenshot white-bg">
    <div class="title">קוד כותרת הפוסט פתוח</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="קוד כותרת הפוסט פתוח" />
</div>

כעת נדביק את קטע הקוד שלנו:

<div class="screenshot white-bg">
    <div class="title">קטע קוד הכותרת הודבק</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="קטע קוד הכותרת הודבק" />
</div>

השלב הבא הוא קוד התחתית (footer) להפעלת FastComments. לחץ על סימן ה-`+` שליד `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">פתח קוד תחתית הפוסט</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="פתח קוד תחתית הפוסט" />
</div>

העתק קטע קוד זה שנועד **במיוחד עבור Weebly**:

[inline-code-attrs-start title = 'קטע קוד לתחתית של FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // הסר את כפתור 'הצג תגובות'
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

כעת נדביק את קוד התחתית שלנו:

<div class="screenshot white-bg">
    <div class="title">קוד התחתית של הפוסט נוסף</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="קוד התחתית של הפוסט נוסף" />
</div>

זהו!