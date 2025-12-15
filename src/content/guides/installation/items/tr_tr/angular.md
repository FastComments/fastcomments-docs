Angular kütüphanemizi NPM'de <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">burada</a> bulabilirsiniz.

FastComments Angular yorum widget'ı, VanillaJS olanla aynı tüm özellikleri destekler - canlı yorumlama, SSO vb.

Bir eş bağımlılık olan fastcomments-typescript'e ihtiyacınız olacak. Lütfen bunun TypeScript derlemenize dahil edildiğinden emin olun.
Gelecekte, bu eş bağımlılık @types/fastcomments'a taşınacak ve kurulum basitleştirilecektir.

[inline-code-attrs-start title = 'FastComments Angular (NPM ile)'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Eş bağımlılık tsconfig.json dosyanıza eklenmelidir, örneğin:

[inline-code-attrs-start title = 'fastcomments-typescript eş bağımlılığı ekleme'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Ardından, uygulamanıza `FastCommentsModule` ekleyin:

[inline-code-attrs-start title = 'Modülü Uygulamanıza Ekleyin'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Kullanım

Başlamak için demo kiracı için bir yapılandırma nesnesi geçiriyoruz:

[inline-code-attrs-start title = 'Kullanım - Satır İçi Yapılandırma'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Yapılandırma oldukça karmaşık olabileceğinden, bir nesne referansı geçirebiliriz:

[inline-code-attrs-start title = 'Kullanım - Yapılandırma için Nesne Geçirme'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Kullanım - AB'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widget değişiklik algılamayı kullanır, bu nedenle yapılandırma nesnesinin herhangi bir özelliğini değiştirmek yeniden yüklenmesine neden olur.

Angular bileşeninin desteklediği yapılandırmayı <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">burada</a> bulabilirsiniz.
