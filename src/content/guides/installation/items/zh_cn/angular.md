您可以在NPM的<a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">这里</a>找到我们的Angular库。

FastComments Angular评论小部件支持与VanillaJS版本相同的所有功能——实时评论、SSO等。

您需要fastcomments-typescript，这是一个对等依赖。请确保它包含在您的TypeScript编译中。
将来，此对等依赖将移至@types/fastcomments，这将简化安装。

[inline-code-attrs-start title = 'FastComments Angular（通过NPM）'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

对等依赖应添加到您的tsconfig.json文件中，例如：

[inline-code-attrs-start title = '添加fastcomments-typescript对等依赖'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

然后，将`FastCommentsModule`添加到您的应用程序：

[inline-code-attrs-start title = '将模块添加到您的应用'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## 使用方法

首先，我们传递演示租户的配置对象：

[inline-code-attrs-start title = '使用方法 - 内联配置'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

由于配置可能变得相当复杂，我们可以传递对象引用：

[inline-code-attrs-start title = '使用方法 - 传递配置对象'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '使用方法 - 欧盟'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

小部件使用变更检测，因此更改配置对象的任何属性都会导致它重新加载。

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">这里</a>找到Angular组件支持的配置。
