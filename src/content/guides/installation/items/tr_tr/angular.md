Angular ile oluşturulmuş bir siteye yorum eklemek için Angular kütüphanemizi NPM'de <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">burada</a> bulabilirsiniz.

FastComments Angular yorum bileşeni VanillaJS sürümüyle aynı tüm özellikleri destekler - canlı yorum, sso vb.

fastcomments-typescript'e ihtiyacınız olacak; bu bir eş bağımlılıktır. Lütfen bunun TypeScript derlemenize dahil edildiğinden emin olun.
Gelecekte bu eş bağımlılık @types/fastcomments'e taşınacak ve bu kurulumu basitleştirecektir.

[inline-code-attrs-start title = 'FastComments Angular - NPM üzerinden'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Eş bağımlılık tsconfig.json dosyanıza eklenmelidir, örneğin:

[inline-code-attrs-start title = 'fastcomments-typescript peer bağımlılığını ekleme'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Daha sonra, `FastCommentsModule`'u uygulamanıza ekleyin:

[inline-code-attrs-start title = 'Uygulamanıza Modülü Ekleyin'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Başlamak için demo tenant için bir yapılandırma (config) nesnesi geçiriyoruz:

[inline-code-attrs-start title = 'Kullanım - Satır İçi Yapılandırma'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Yapılandırma oldukça karmaşık hale gelebileceğinden, bir nesne referansı geçebiliriz:

[inline-code-attrs-start title = 'Kullanım - Yapılandırma için Nesne Geçirme'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Kullanım - AB'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Bileşen değişiklik algılama kullanır, bu yüzden yapılandırma nesnesinin herhangi bir özelliğini değiştirmek bileşenin yeniden yüklenmesine neden olacaktır.

Angular bileşeninin desteklediği yapılandırmayı <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">burada</a> bulabilirsiniz.

---