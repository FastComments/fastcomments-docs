כדי לראות כל ווידג'ט וכל זרימה שרצים באופן מקומי מול ה־tenant הציבורי `demo`, שיבטו את המאגר והריצו:

```bash
yarn bootstrap
cd example
yarn ios       # או: yarn android, yarn web
```

נקודת הכניסה של תצוגת ההדגמה היא `example/src/ShowcaseApp.tsx` — אפליקציה אחת שמציגה את כל הווידג'טים, ערכות הנושא והזרימות.

היעד `yarn web` משתמש ב־`react-native-web` + `react-native-web-webview` (שמציג את ה־WebView כ־iframe). שימושי לבדיקות חזותיות מהירות בדפדפן; ממשקי ה־WebView המיועדים לנייטיב בלבד כמו `injectJavaScript` ו־`onShouldStartLoadWithRequest` לא יתפקדו במלואם בגרסת ה־web.