За добавяне на коментари в сайт, създаден с Angular, можете да намерите нашата Angular библиотека в NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">тук</a>.

Компонентът за коментари FastComments за Angular поддържа всички същите функции като VanillaJS версията - live commenting, sso, и т.н.

Ще ви трябва fastcomments-typescript, който е peer dependency. Моля, уверете се, че това е включено в компилацията ви на TypeScript.
В бъдеще тази peer dependency ще бъде преместена в @types/fastcomments, което ще опрости тази инсталация.

[inline-code-attrs-start title = 'FastComments за Angular чрез NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Този peer dependency трябва да бъде добавен във вашия tsconfig.json файл, например:

[inline-code-attrs-start title = 'Добавяне на peer dependency fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

След това добавете `FastCommentsModule` към вашето приложение:

[inline-code-attrs-start title = 'Добавете модула към приложението си'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Използване

За да започнете, подаваме конфигурационен обект за демонстрационния tenant:

[inline-code-attrs-start title = 'Използване - Вградена конфигурация'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Тъй като конфигурацията може да стане доста сложна, можем да предадем референтен обект:

[inline-code-attrs-start title = 'Използване - Предаване на обект за конфигурация'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Използване - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Уиджът използва откриване на промени (change detection), така че промяната на произволно свойство на конфигурационния обект ще го презареди.

Можете да намерите конфигурацията, която Angular компонентът поддържа <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тук</a>.

---