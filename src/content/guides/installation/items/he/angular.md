להוספת תגובות לאתר שנבנה עם Angular, ניתן למצוא את ספריית ה-Angular שלנו ב-NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">כאן</a>.

ווידג'ט התגובות של FastComments ל-Angular תומך בכל אותן התכונות של ה-VanillaJS — תגובות בזמן אמת, sso, ועוד.

תזדקקו ל-fastcomments-typescript, שהיא peer dependency. אנא ודאו שהיא כלולה בקומפילציית TypeScript שלכם.
בעתיד, ה-peer dependency הזה יעבור ל-@types/fastcomments מה שיפשט את ההתקנה הזו.

[inline-code-attrs-start title = 'FastComments Angular דרך NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

יש להוסיף את ה-peer dependency בקובץ tsconfig.json שלכם, לדוגמה:

[inline-code-attrs-start title = 'הוספת peer dependency של fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

לאחר מכן, הוסיפו את `FastCommentsModule` לאפליקציה שלכם:

[inline-code-attrs-start title = 'הוספת המודול לאפליקציה שלכם'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

כדי להתחיל, נעביר אובייקט config עבור ה-tenant demo:

[inline-code-attrs-start title = 'שימוש - קונפיגורציה מוטמעת'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

מאחר שהקונפיגורציה עלולה להיות מורכבת, נוכל להעביר הפניה לאובייקט:

[inline-code-attrs-start title = 'שימוש - העברת אובייקט לקונפיגורציה'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'שימוש - אירופה (EU)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

הווידג'ט משתמש בזיהוי שינויים, ולכן שינוי של כל אחת מהתכונות של אובייקט הקונפיגורציה יגרום לטעינה מחדש שלו.

ניתן למצוא את הקונפיגורציה שהקומפוננטה של Angular תומכת בה <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.

---