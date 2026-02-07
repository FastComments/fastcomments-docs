Angularで構築されたサイトにコメントを追加するには、当社のAngularライブラリをNPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">こちら</a> で見つけることができます。

FastComments の Angular コメントウィジェットは、VanillaJS のものと同じ機能（ライブコメント、sso など）をサポートしています。

fastcomments-typescript がピア依存関係として必要です。これが TypeScript のコンパイルに含まれていることを確認してください。将来的には、このピア依存関係は @types/fastcomments に移され、インストールが簡素化されます。

[inline-code-attrs-start title = 'NPM経由のFastComments Angular'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

このピア依存関係は tsconfig.json ファイルに追加する必要があります。例:

[inline-code-attrs-start title = 'fastcomments-typescript ピア依存関係の追加'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

次に、`FastCommentsModule` をアプリケーションに追加します:

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

開始するには、デモテナント用のconfigオブジェクトを渡します:

[inline-code-attrs-start title = '使用 - インライン構成'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

構成が複雑になることがあるため、オブジェクト参照を渡すこともできます:

[inline-code-attrs-start title = '使用 - 構成にオブジェクトを渡す'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '使用 - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

ウィジェットは変更検出を使用するため、構成オブジェクトの任意のプロパティを変更するとウィジェットが再読み込みされます。

Angularコンポーネントがサポートする構成は <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a> で確認できます。

---