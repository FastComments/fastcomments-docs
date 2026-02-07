Щоб додати коментарі на сайт, створений за допомогою Angular, ви можете знайти нашу бібліотеку Angular на NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">тут</a>.

Компонент коментування FastComments для Angular підтримує всі ті ж функції, що й версія на VanillaJS — коментування в режимі реального часу, SSO тощо.

Вам знадобиться fastcomments-typescript, яка є peer-залежністю. Будь ласка, переконайтеся, що вона включена у вашу компіляцію TypeScript.
У майбутньому ця peer-залежність буде перенесена до @types/fastcomments, що спростить встановлення.

[inline-code-attrs-start title = 'FastComments для Angular через NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Цю peer-залежність слід додати у ваш файл tsconfig.json, наприклад:

[inline-code-attrs-start title = 'Додавання peer-залежності fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Потім додайте `FastCommentsModule` до вашого застосунку:

[inline-code-attrs-start title = 'Додайте модуль у ваш додаток'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Використання

Щоб почати, ми передаємо об'єкт конфігурації для демо-тенанта:

[inline-code-attrs-start title = 'Використання - інлайн-конфігурація'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Оскільки конфігурація може стати досить складною, ми можемо передати посилання на об'єкт:

[inline-code-attrs-start title = 'Використання - передача об'єкта конфігурації'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Використання - ЄС'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Віджет використовує відстеження змін, тож зміна будь-якої властивості об'єкта конфігурації спричинить його перезавантаження.

Ви можете знайти конфігурацію, яку підтримує Angular-компонент, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тут</a>.

---