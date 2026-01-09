עבור Page Reacts עלינו להחליט על שתי נקודות:

- אילו תמונות תגובה להשתמש.
- `id` קצר לשם כל תגובה.

באופן אופציונלי:

- ניתן גם להגדיר תמונות נפרדות אופציונליות לתגובות שנבחרו/לא נבחרו.
- ניתן להחליט אם להציג את רשימת המשתמשים שהגיבו בעת העברת העכבר מעל אחת מהתגובות. 

[inline-code-attrs-start title = 'דוגמת קוד ל-Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.FastCommentsUI(document.getElementById('page-reacts-example'), {
        tenantId: 'demo',
        pageReactConfig: {
            showUsers: true,
            reacts: [
                {id: 'droll', src: 'https://docs.fastcomments.com/images/emojis/droll.png'},
                {id: 'he', src: 'https://docs.fastcomments.com/images/emojis/heart-eyes.png'},
                {id: 'laugh', src: 'https://docs.fastcomments.com/images/emojis/laugh.png'},
                {id: 'puke', src: 'https://docs.fastcomments.com/images/emojis/puke.png', selectedSrc: 'https://docs.fastcomments.com/images/emojis/puke-bw.png' },
                {id: 'rofl', src: 'https://docs.fastcomments.com/images/emojis/rofl.png' },
            ]
        }
    });
</script>
[inline-code-end]

ההגדרה עבור ספריות ה־frontend כמו React, Angular וכו' זהה.

---