AngularライブラリはNPMの<a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">こちら</a>で見つけることができます。

FastComments Angularコメントウィジェットは、VanillaJS版と同じすべての機能（ライブコメント、SSO など）をサポートしています。

ピア依存関係であるfastcomments-typescriptが必要です。TypeScriptコンパイルに含まれていることを確認してください。
将来的には、このピア依存関係は@types/fastcommentsに移動され、インストールが簡素化されます。

[inline-code-attrs-start title = 'FastComments Angular（NPM経由）'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

ピア依存関係はtsconfig.jsonファイルに追加する必要があります。例：

[inline-code-attrs-start title = 'fastcomments-typescriptピア依存関係の追加'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

次に、アプリケーションに`FastCommentsModule`を追加します：

[inline-code-attrs-start title = 'アプリにモジュールを追加'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

開始するには、デモテナント用の設定オブジェクトを渡します：

[inline-code-attrs-start title = '使用方法 - インライン設定'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

設定はかなり複雑になる可能性があるため、オブジェクト参照を渡すことができます：

[inline-code-attrs-start title = '使用方法 - 設定用オブジェクトを渡す'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '使用方法 - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

ウィジェットは変更検出を使用するため、設定オブジェクトのプロパティを変更するとリロードされます。

Angularコンポーネントがサポートする設定は<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a>で見つけることができます。
