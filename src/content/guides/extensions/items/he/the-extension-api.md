האינטראקציה עם ה-`Extension` פשוטה, מאחר שאנו מגדירים הפניות לפונקציות שאנחנו רוצים שיופעלו.

כדי להמשיך מהדוגמה הקודמת, נניח שאנו רוצים להוסיף HTML לראש כל תגובה:

[inline-code-attrs-start title = 'תוסף פשוט - המשך'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    extension.commentFilter = function(comment, html) {
        return `<h3>The user's name is ${comment.commenterName}!</h3>` + html;
    }
})();
[inline-code-end]

כאשר תחזיר HTML כזה, הוא יתמזג בממשק המשתמש באמצעות אלגוריתם dom-diffing.

#### הפעלת הצגה מחדש של תגובה באופן ידני

אפשר להמתין לטעינת הדף ההתחלתית ולהציג תגובה מחדש באופן ידני על-ידי קריאה ל-`reRenderComment`:

[inline-code-attrs-start title = 'הצגה מחדש של תגובה'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    let renderCount = 0;

    extension.commentFilter = function(comment, html) {
        renderCount++;
        return `<h3>The render count is ${renderCount}!</h3>` + html;
    }

    extension.onInitialRenderComplete = function() {
        setInterval(function() {
            extension.reRenderComment(extension.commentsById[Object.keys(extension.commentsById)[0]], function renderDone() {
                console.log('Comment re-render done.');
            });
        }, 2000); // אין צורך ב-timeout, זו רק דוגמה.
    }
})();
[inline-code-end]

---