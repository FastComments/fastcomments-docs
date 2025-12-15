Ви можете знайти нашу бібліотеку Angular на NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">тут</a>.

Віджет коментарів FastComments для Angular підтримує всі ті ж функції, що й VanillaJS версія — коментування в реальному часі, SSO тощо.

Вам знадобиться fastcomments-typescript, який є peer-залежністю. Будь ласка, переконайтеся, що він включений у вашу компіляцію TypeScript.
У майбутньому ця peer-залежність буде переміщена до @types/fastcomments, що спростить встановлення.

[inline-code-attrs-start title = 'FastComments Angular через NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer-залежність повинна бути додана у ваш файл tsconfig.json, наприклад:

[inline-code-attrs-start title = 'Додавання peer-залежності fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Потім додайте `FastCommentsModule` у ваш застосунок:

[inline-code-attrs-start title = 'Додавання модуля у ваш застосунок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Для початку ми передаємо об'єкт конфігурації для демо-тенанта:

[inline-code-attrs-start title = 'Використання - Вбудована конфігурація'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Оскільки конфігурація може бути досить складною, ми можемо передати посилання на об'єкт:

[inline-code-attrs-start title = 'Використання - Передача об\'єкта для конфігурації'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Використання - ЄС'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Віджет використовує виявлення змін, тому зміна будь-яких властивостей об'єкта конфігурації призведе до його перезавантаження.

Ви можете знайти конфігурацію, яку підтримує компонент Angular <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тут</a>.
