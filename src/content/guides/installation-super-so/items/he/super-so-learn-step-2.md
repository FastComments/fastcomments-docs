בשלב הבא עליך להעתיק את קוד הווידג'ט המוכן מטה.

כל עוד אתה מחובר ל-FastComments.com הקטע הקוד למטה כבר יכלול את מידע החשבון שלך. בוא נעתיק אותו:

[inline-code-attrs-start title = 'קוד Super.so של FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // ניקוי מופע קיים
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // הסר את סרגל העליון הקיים אם קיים
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // צור סרגל עליון חדש
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // אתחול FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // עדכן את currentPathname
            currentPathname = window.location.pathname;
        }

        // טעינה ראשונית
        load();

        // בדוק כל 500ms אם יש שינויים
        setInterval(() => {
            // טען מחדש אם ה-pathname השתנה
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // טען מחדש אם הווידג'ט הוסר
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // טען מחדש אם המכולה התרוקנה
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Now paste in the `Body` area:

<div class="screenshot white-bg">
    <div class="title">הקוד שהוצמד</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="הקוד שהוצמד" />
</div>

If you see a "this is a demo message" after pasting the code:

- Ensure you're logged into your fastcomments.com account.
- Ensure you have 3rd party cookies enabled.
- Then refresh this page and copy the code snippet again. It should have `tenantId` populated with your tenant's identifier.