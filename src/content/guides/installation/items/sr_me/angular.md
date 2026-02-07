За додавање коментара на сајт изграђен помоћу Angular-а, нашу Angular библиотеку можете наћи на NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">овде</a>.

FastComments Angular коментарски видгет подржава све исте функције као и VanillaJS верзија — коментарисање у стварном времену, SSO и тако даље.

Биће вам потребан fastcomments-typescript, који је peer зависност. Молимо осигурајте да је ово укључено у вашу TypeScript компилацију.
У будућности, ова peer зависност биће премештена у @types/fastcomments што ће поједноставити ову инсталацију.

[inline-code-attrs-start title = 'FastComments Angular преко NPM-а'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer зависност треба додати у ваш tsconfig.json фајл, на пример:

[inline-code-attrs-start title = 'Додавање fastcomments-typescript peer зависности'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Затим, додајте `FastCommentsModule` у вашу апликацију:

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

## Употреба

За почетак, прослеђујемо конфигурациони објекат за демо тенанта:

[inline-code-attrs-start title = 'Употреба - Инлајн конфигурација'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Пошто конфигурација може постати прилично сложена, можемо проследити референцу на објекат:

[inline-code-attrs-start title = 'Употреба - Проследите објекат за конфигурацију'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Употреба - ЕУ'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Видгет користи детекцију промена, тако да промена било које особине конфигурационог објекта ће узроковати његово поновно учитавање.

Конфигурацију коју Angular компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.