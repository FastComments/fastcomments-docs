要在使用 Angular 建置的網站中加入評論，您可以在 NPM 找到我們的 Angular 函式庫 <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">這裡</a>。

FastComments 的 Angular 評論元件支援與 VanillaJS 相同的所有功能 — 即時評論、單一登入（SSO）等等。

您需要 fastcomments-typescript，它是一個 peer 相依。請確保此相依已包含在您的 TypeScript 編譯中。
未來，此 peer 相依將移至 @types/fastcomments，這會簡化安裝程序。

[inline-code-attrs-start title = '透過 NPM 安裝 FastComments Angular'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

這個 peer 相依應該被加入到您的 tsconfig.json 檔案中，例如：

[inline-code-attrs-start title = '新增 fastcomments-typescript peer 相依'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

接著，將 `FastCommentsModule` 新增到您的應用程式中：

[inline-code-attrs-start title = '將模組新增到您的應用程式'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## 使用

要開始，傳入示範租戶的設定物件：

[inline-code-attrs-start title = '使用 - 內聯設定'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

由於設定可能變得相當複雜，我們也可以傳入物件參考：

[inline-code-attrs-start title = '使用 - 傳入物件作為設定'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '使用 - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

該元件使用變更偵測，因此變更設定物件的任何屬性都會導致其重新載入。

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a> 找到 Angular 元件所支援的設定。