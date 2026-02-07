Angular로 구축된 사이트에 댓글을 추가하려면, NPM에서 우리의 Angular 라이브러리를 <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">여기</a>에서 찾을 수 있습니다.

FastComments Angular 댓글 위젯은 VanillaJS 위젯과 동일한 모든 기능(라이브 댓글, SSO 등)을 지원합니다.

fastcomments-typescript가 피어 의존성으로 필요합니다. 이 패키지가 TypeScript 컴파일에 포함되어 있는지 확인하세요.
향후 이 피어 의존성은 설치를 단순화하는 @types/fastcomments로 이동될 예정입니다.

[inline-code-attrs-start title = 'NPM을 통한 FastComments Angular'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

피어 의존성은 예를 들어 tsconfig.json 파일에 추가되어야 합니다:

[inline-code-attrs-start title = 'fastcomments-typescript 피어 의존성 추가'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

그런 다음, 애플리케이션에 `FastCommentsModule`을 추가하세요:

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

## 사용

시작하려면 데모 테넌트에 대한 구성 객체를 전달합니다:

[inline-code-attrs-start title = '사용 - 인라인 구성'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

구성이 복잡해질 수 있으므로 객체 참조를 전달할 수 있습니다:

[inline-code-attrs-start title = '사용 - 구성 객체 전달'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = '사용 - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

위젯은 변경 감지를 사용하므로 구성 객체의 어떤 속성이든 변경되면 위젯이 다시 로드됩니다.

Angular 컴포넌트가 지원하는 구성은 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">여기</a>에서 확인할 수 있습니다.

---