Нашу Angular библиотеку можете пронаћи на NPM-у <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">овде</a>.

FastComments Angular виџет за коментаре подржава све исте функције као VanillaJS верзија — коментарисање у реалном времену, SSO и тако даље.

Требаће вам fastcomments-typescript, који је peer зависност. Молимо осигурајте да је укључен у вашу TypeScript компилацију.
У будућности ће ова peer зависност бити премештена у @types/fastcomments што ће поједноставити инсталацију.

[inline-code-attrs-start title = 'FastComments Angular преко NPM-а'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer зависност треба додати у вашу tsconfig.json датотеку, на пример:

[inline-code-attrs-start title = 'Додавање fastcomments-typescript peer зависности'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Затим додајте `FastCommentsModule` у вашу апликацију:

[inline-code-attrs-start title = 'Додајте модул у вашу апликацију'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Коришћење

За почетак, прослеђујемо конфигурацијски објекат за демо закупца:

[inline-code-attrs-start title = 'Коришћење - Inline конфигурација'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Пошто конфигурација може постати прилично сложена, можемо проследити референцу објекта:

[inline-code-attrs-start title = 'Коришћење - Проследи објекат за конфигурацију'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Коришћење - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Виџет користи детекцију промена, тако да ће промена било којих својстава конфигурацијског објекта узроковати поновно учитавање.

Конфигурацију коју Angular компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.
