אתה יכול למצוא את ספריית ה-Angular שלנו ב-NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">כאן</a>.

ווידג'ט התגובות של FastComments ל-Angular תומך בכל אותן התכונות של גרסת VanillaJS - תגובות בזמן אמת, SSO, ועוד.

תצטרך את fastcomments-typescript, שהיא peer dependency. אנא ודא שהיא כלולה בקומפילציית ה-TypeScript שלך.
בעתיד, peer dependency זו תועבר ל-@types/fastcomments שיפשט את ההתקנה.

[inline-code-attrs-start title = 'FastComments Angular דרך NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

יש להוסיף את ה-peer dependency בקובץ tsconfig.json שלך, לדוגמה:

[inline-code-attrs-start title = 'הוספת fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

לאחר מכן, הוסף את `FastCommentsModule` לאפליקציה שלך:

[inline-code-attrs-start title = 'הוסף את המודול לאפליקציה שלך'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppComponent } from './app.component';
import { FastCommentsModule } from 'ngx-fastcomments';

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    FastCommentsModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
[inline-code-end]

## שימוש

כדי להתחיל, אנו מעבירים אובייקט הגדרות עבור הדייר הדמו:

[inline-code-attrs-start title = 'שימוש - הגדרה Inline'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

מכיוון שההגדרות יכולות להיות מורכבות למדי, אנו יכולים להעביר הפניה לאובייקט:

[inline-code-attrs-start title = 'שימוש - העברת אובייקט להגדרות'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'שימוש - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

הווידג'ט משתמש בזיהוי שינויים, כך ששינוי כל מאפיינים של אובייקט ההגדרות יגרום לטעינה מחדש.

אתה יכול למצוא את ההגדרות שהקומפוננטה של Angular תומכת בהן <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.
