要在基于 Angular 构建的网站中添加评论，您可以在 NPM 上找到我们的 Angular 库，链接在 <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">这里</a>。

FastComments Angular 评论小部件支持与 VanillaJS 相同的所有功能 —— 实时评论、单点登录 (SSO) 等。

您将需要 fastcomments-typescript，它是一个 peer 依赖。请确保此依赖包含在您的 TypeScript 编译中。未来，该 peer 依赖将迁移到 @types/fastcomments，从而简化此安装。

[inline-code-attrs-start title = '通过 NPM 安装 FastComments Angular'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

该 peer 依赖应添加到您的 tsconfig.json 文件中，例如：

[inline-code-attrs-start title = '在 tsconfig.json 中添加 fastcomments-typescript 同伴依赖'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

然后，将 `FastCommentsModule` 添加到您的应用中：

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

## 用法

要开始使用，我们为演示租户传入一个配置对象：

[inline-code-attrs-start title = '用法 - 行内配置'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

由于配置可能相当复杂，我们可以传入对象引用：

[inline-code-attrs-start title = '用法 - 传入对象进行配置'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '用法 - EU 区域'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

该小部件使用变更检测，因此更改配置对象的任何属性将导致其重新加载。

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">此处</a> 找到 Angular 组件支持的配置。 

---