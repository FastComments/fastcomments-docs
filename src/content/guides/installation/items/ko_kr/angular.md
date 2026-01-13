Angular 라이브러리는 NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">여기</a>에서 찾을 수 있습니다.

FastComments Angular 댓글 위젯은 VanillaJS 버전과 동일한 모든 기능(실시간 댓글, SSO 등)을 지원합니다.

피어 종속성인 fastcomments-typescript가 필요합니다. TypeScript 컴파일에 포함되어 있는지 확인하세요.
향후 이 피어 종속성은 @types/fastcomments로 이동되어 설치가 간소화될 예정입니다.

[inline-code-attrs-start title = 'FastComments Angular (NPM 사용)'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

피어 종속성은 tsconfig.json 파일에 추가해야 합니다. 예:

[inline-code-attrs-start title = 'fastcomments-typescript 피어 종속성 추가'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

그런 다음 애플리케이션에 `FastCommentsModule`을 추가합니다:

[inline-code-attrs-start title = '앱에 모듈 추가'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## 사용법

시작하려면 데모 테넌트용 구성 객체를 전달합니다:

[inline-code-attrs-start title = '사용법 - 인라인 구성'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

구성이 상당히 복잡해질 수 있으므로 객체 참조를 전달할 수 있습니다:

[inline-code-attrs-start title = '사용법 - 구성용 객체 전달'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '사용법 - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

위젯은 변경 감지를 사용하므로 구성 객체의 속성을 변경하면 다시 로드됩니다.

Angular 컴포넌트가 지원하는 구성은 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">여기</a>에서 찾을 수 있습니다.
