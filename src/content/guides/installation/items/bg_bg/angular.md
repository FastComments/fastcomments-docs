Можете да намерите нашата Angular библиотека в NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">тук</a>.

FastComments Angular уиджетът за коментари поддържа всички същите функции като VanillaJS версията — коментиране в реално време, SSO и т.н.

Ще ви трябва fastcomments-typescript, която е peer зависимост. Моля, уверете се, че е включена в компилацията на TypeScript.
В бъдеще тази peer зависимост ще бъде преместена в @types/fastcomments, което ще опрости инсталацията.

[inline-code-attrs-start title = 'FastComments Angular чрез NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer зависимостта трябва да бъде добавена във вашия tsconfig.json файл, например:

[inline-code-attrs-start title = 'Добавяне на fastcomments-typescript peer зависимост'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

След това добавете `FastCommentsModule` към вашето приложение:

[inline-code-attrs-start title = 'Добавяне на модула към вашето приложение'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

За начало подаваме конфигурационен обект за демо тенанта:

[inline-code-attrs-start title = 'Използване - Вградена конфигурация'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Тъй като конфигурацията може да стане доста сложна, можем да подадем референция към обект:

[inline-code-attrs-start title = 'Използване - Подаване на обект за конфигурация'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Използване - ЕС'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Уиджетът използва откриване на промени, така че промяната на всякакви свойства на конфигурационния обект ще доведе до презареждане.

Можете да намерите конфигурацията, която Angular компонентът поддържа <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тук</a>.
