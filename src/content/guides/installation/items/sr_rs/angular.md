За додавање коментара на сајт изграђен са Angular-ом, нашу Angular библиотеку можете пронаћи на NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">овде</a>.

FastComments Angular виџет за коментаре подржава све исте функције као VanillaJS верзија - коментаре у реалном времену, sso и тако даље.

Потребан вам је fastcomments-typescript, који је peer dependency. Молимо вас да уверите да је укључен у вашу TypeScript компилацију.
У будућности, ова peer dependency биће премештена у @types/fastcomments што ће поједноставити инсталацију.

[inline-code-attrs-start title = 'FastComments Angular преко NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer dependency треба додати у вашу tsconfig.json датотеку, на пример:

[inline-code-attrs-start title = 'Додавање fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Затим додајте `FastCommentsModule` у вашу апликацију:

[inline-code-attrs-start title = 'Додавање модула у вашу апликацију'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Употреба

Да бисте започели, прослеђујемо конфигурациони објекат за demo tenant:

[inline-code-attrs-start title = 'Употреба - Инлине конфигурација'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Пошто конфигурација може постати прилично сложена, можемо проследити референцу на објекат:

[inline-code-attrs-start title = 'Употреба - Прослеђивање објекта за конфигурацију'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Употреба - ЕУ'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Виџет користи change detection, тако да промена било које особине конфигурационог објекта доводи до његовог поновног учитавања.

Конфигурацију коју Angular компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.