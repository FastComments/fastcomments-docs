ספרייה זו היא מימוש מלא של react-native עבור [FastComments](https://fastcomments.com).

היא תומכת ב-live commenting, chat, threads, emoticons, notifications, SSO, skins, ובמִתְאִּימוּת מלאה על-ידי העברת stylesheet object. כל ה-assets
יכולים גם להיות מותאמים, והיא תומכת בהחלפה בין assets שונים בהתבסס על dark mode.

היתרון של ספרייה זו הוא שהיא גמישה יותר, ואינה דורשת webview, כמו ה-wrapper `fastcomments-react-native`.

הכל רץ על ה-backend של FastComments, כך שעליך לשלב רק את ה-UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

ראה את [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) לעוד דוגמאות.

הוסיפו live chat לאפליקציית React Native הקיימת שלכם, או אפילו בנו רשת חברתית!