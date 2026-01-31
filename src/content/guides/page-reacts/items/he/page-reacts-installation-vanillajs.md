---
לגבי תגובות בעמוד עלינו להחליט שתי נקודות:

- אילו תמונות תגובה להשתמש בהן.
- מזהה קצר (`id`) לשם כל תגובה.

באופן אופציונלי:

- ניתן גם להגדיר תמונות נפרדות אופציונליות עבור תגובות שנבחרו/לא נבחרו.
- ניתן להחליט אם להציג את רשימת המשתמשים שהגיבו כאשר מעבירים את העכבר מעל אחת מהתגובות. 

[inline-code-attrs-start title = 'דוגמת קוד תגובות בעמוד'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.fcConfigs = [{
        target: '#page-reacts-example',
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
    }];
</script>
[inline-code-end]

התצורה עבור ספריות ה־frontend כמו React, Angular וכו' זהה.

---