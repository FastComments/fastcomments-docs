Для добавления комментариев на сайт, созданный с помощью Angular, вы можете найти нашу библиотеку Angular на NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">здесь</a>.

Компонент комментариев FastComments для Angular поддерживает все те же функции, что и версия на VanillaJS — live-комментирование, sso и т. д.

Вам потребуется fastcomments-typescript, который является peer-зависимостью. Пожалуйста, убедитесь, что она включена в вашу компиляцию TypeScript.
В будущем эта peer-зависимость будет перемещена в @types/fastcomments, что упростит установку.

[inline-code-attrs-start title = 'FastComments Angular через NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer-зависимость должна быть добавлена в ваш файл tsconfig.json, например:

[inline-code-attrs-start title = 'Добавление peer-зависимости fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Затем добавьте `FastCommentsModule` в ваше приложение:

[inline-code-attrs-start title = 'Добавление модуля в приложение'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Использование

Чтобы начать, мы передаём объект конфигурации для демо-тенанта:

[inline-code-attrs-start title = 'Использование - Встроенная конфигурация'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Поскольку конфигурация может быть довольно сложной, мы можем передать ссылку на объект:

[inline-code-attrs-start title = 'Использование - Передача объекта конфигурации'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Использование - ЕС'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Виджет использует обнаружение изменений, поэтому изменение любых свойств объекта конфигурации приведёт к его перезагрузке.

Вы можете найти конфигурацию, которую поддерживает Angular-компонент, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">здесь</a>.

---