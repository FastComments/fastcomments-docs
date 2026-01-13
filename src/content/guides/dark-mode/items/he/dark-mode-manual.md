### למפתחים - כפיית כיבוי מצב כהה

כפיית כיבוי מצב כהה יכולה להיעשות על ידי העברת `hasDarkBackground` כ-`false` בתצורת הוידג'ט. זה עובד עבור ספריות VanillaJS, Angular, React, Vue ו-React Native.

לכל ספרייה יש תיקיית `examples` ב-[GitHub](https://github.com/fastComments/) שמכילה דוגמאות לשימוש במצב כהה.

### כפיית הפעלת מצב כהה

אנחנו יכולים לכפות שמצב כהה יהיה תמיד מופעל על ידי הגדרת `hasDarkBackground` ל-`true`.

אנחנו יכולים גם לעשות זאת דרך ממשק התאמת הוידג'ט [כאן](https://fastcomments.com/auth/my-account/customize-widget).

תחת `Base Theme` פשוט בחרו `Force Dark Mode`.

### וידג'ט VanillaJS - עדכון מצב כהה

הדרך הקלה ביותר לעדכן מצב כהה היא לעבור על כל המופעים של הוידג'ט בדף ולעדכן את התצורה שלהם:

[inline-code-attrs-start title = 'VanillaJS - Dark Mode Toggle'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    let isDarkMode = somehowDetermineIfDarkModeEnabled();
    for (const instanceWrapped of window.fcUIInstances) {
        if (instanceWrapped.targetElement) {
            const config = instanceWrapped.config;
            config.hasDarkBackground = isDarkMode;
            instanceWrapped.instance.update(config)
        }
    }
[inline-code-end]
