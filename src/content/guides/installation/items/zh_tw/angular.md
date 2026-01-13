您可以在NPM的<a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">這裡</a>找到我們的Angular函式庫。

FastComments Angular評論小工具支援與VanillaJS版本相同的所有功能——即時評論、SSO等。

您需要fastcomments-typescript，這是一個對等依賴。請確保它包含在您的TypeScript編譯中。
將來，此對等依賴將移至@types/fastcomments，這將簡化安裝。

[inline-code-attrs-start title = 'FastComments Angular（透過NPM）'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

對等依賴應添加到您的tsconfig.json檔案中，例如：

[inline-code-attrs-start title = '添加fastcomments-typescript對等依賴'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

然後，將`FastCommentsModule`添加到您的應用程式：

[inline-code-attrs-start title = '將模組添加到您的應用'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

首先，我們傳遞示範租戶的配置物件：

[inline-code-attrs-start title = '使用方法 - 內聯配置'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

由於配置可能變得相當複雜，我們可以傳遞物件參考：

[inline-code-attrs-start title = '使用方法 - 傳遞配置物件'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '使用方法 - 歐盟'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

小工具使用變更偵測，因此更改配置物件的任何屬性都會導致它重新載入。

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a>找到Angular元件支援的配置。
