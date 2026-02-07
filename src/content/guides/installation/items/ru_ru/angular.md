Для добавления комментариев на сайт, созданный с использованием Angular, вы можете найти нашу библиотеку Angular на NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">здесь</a>.

Виджет комментирования FastComments для Angular поддерживает все те же возможности, что и версия на VanillaJS — живое комментирование, sso и т.д.

Вам потребуется fastcomments-typescript, который является peer-зависимостью. Пожалуйста, убедитесь, что он включён в вашу компиляцию TypeScript.
В будущем эта peer-зависимость будет перенесена в @types/fastcomments, что упростит эту установку.

[inline-code-attrs-start title = 'FastComments Angular через NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Плеер-зависимость должна быть добавлена в ваш tsconfig.json файл, например:

[inline-code-attrs-start title = 'Добавление peer-зависимости fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Затем добавьте `FastCommentsModule` в ваше приложение:

[inline-code-attrs-start title = 'Добавление модуля в ваше приложение'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Для начала мы передаём объект конфигурации для демонстрационного tenant:

[inline-code-attrs-start title = 'Использование - Встроенная конфигурация'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Поскольку конфигурация может быть довольно сложной, мы можем передавать ссылку на объект:

[inline-code-attrs-start title = 'Использование - Передача объекта для конфигурации'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Использование - ЕС'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Виджет использует обнаружение изменений, поэтому изменение любых свойств объекта конфигурации приведёт к его перезагрузке.

Конфигурацию, которую поддерживает Angular-компонент, можно найти <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">здесь</a>.

---