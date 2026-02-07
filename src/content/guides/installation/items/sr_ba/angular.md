Za dodavanje komentara na sajt izgrađen sa Angular-om, našu Angular biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">ovdje</a>.

FastComments Angular komentarski widget podržava sve iste funkcije kao i VanillaJS - live komentarisanje, SSO i slično.

Trebaće vam fastcomments-typescript, koji je peer zavisnost. Molimo osigurajte da je ovo uključeno u vašu TypeScript kompilaciju.
U budućnosti, ova peer zavisnost će biti premještena u @types/fastcomments što će pojednostaviti ovu instalaciju.

[inline-code-attrs-start title = 'FastComments Angular preko NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer zavisnost treba biti dodana u vaš tsconfig.json fajl, na primjer:

[inline-code-attrs-start title = 'Dodavanje fastcomments-typescript peer zavisnosti'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Zatim, dodajte `FastCommentsModule` u vašu aplikaciju:

[inline-code-attrs-start title = 'Dodajte modul u vašu aplikaciju'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Upotreba

Da biste započeli, prosljeđujemo config objekat za demo tenant:

[inline-code-attrs-start title = 'Upotreba - Inline konfiguracija'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Pošto konfiguracija može postati prilično složena, možemo proslijediti referencu na objekat:

[inline-code-attrs-start title = 'Upotreba - Proslijedite objekt za konfiguraciju'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Upotreba - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widget koristi detekciju promjena, tako da promjena bilo koje svojine konfiguracionog objekta prouzrokuje njegovo ponovno učitavanje.

Konfiguraciju koju Angular komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.

---