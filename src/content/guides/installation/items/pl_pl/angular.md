Naszą bibliotekę Angular możesz znaleźć na NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">tutaj</a>.

Widget komentarzy FastComments dla Angular obsługuje wszystkie te same funkcje co wersja VanillaJS - komentarze na żywo, SSO i więcej.

Będziesz potrzebować fastcomments-typescript, która jest zależnością peer. Upewnij się, że jest uwzględniona w kompilacji TypeScript.
W przyszłości ta zależność peer zostanie przeniesiona do @types/fastcomments, co uprości tę instalację.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Zależność peer powinna być dodana w pliku tsconfig.json, na przykład:

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Następnie dodaj `FastCommentsModule` do swojej aplikacji:

[inline-code-attrs-start title = 'Add The Module to Your App'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Użycie

Aby rozpocząć, przekazujemy obiekt konfiguracji dla tenanta demo:

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Ponieważ konfiguracja może być dość skomplikowana, możemy przekazać referencję do obiektu:

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widget używa wykrywania zmian, więc zmiana dowolnych właściwości obiektu konfiguracji spowoduje jego przeładowanie.

Konfigurację obsługiwaną przez komponent Angular możesz znaleźć <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.
