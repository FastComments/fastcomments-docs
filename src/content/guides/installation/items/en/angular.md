For adding comments to a site built with Angular, you can find our Angular library on NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">here</a>.

The FastComments Angular commenting widget supports all of the same features of the VanillaJS one - live commenting, sso, and so on.

You will need fastcomments-typescript, which is a peer dependency. Please ensure this is included in your TypeScript compilation.
In the future, this peer dependency will be moved to @types/fastcomments which will simplify this installation.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

The peer dependency should be added in your tsconfig.json file, for example:

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Then, add the `FastCommentsModule` to your application:

[inline-code-attrs-start title = 'Add The Module to Your App'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Usage

To get started, we pass a config object for the demo tenant:

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Since the configuration can get quite complicated, we can pass in an object reference:

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

The widget uses change detection, so changing any properties of the configuration object will cause it to be reloaded.

You can find the configuration the Angular component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.
