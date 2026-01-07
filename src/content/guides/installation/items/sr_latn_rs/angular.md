Našu Angular biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">ovde</a>.

FastComments Angular vidžet za komentare podržava sve iste funkcije kao VanillaJS verzija — komentarisanje u realnom vremenu, SSO i tako dalje.

Trebaće vam fastcomments-typescript, koji je peer zavisnost. Molimo osigurajte da je uključen u vašu TypeScript kompilaciju.
U budućnosti će ova peer zavisnost biti premeštena u @types/fastcomments što će pojednostaviti instalaciju.

[inline-code-attrs-start title = 'FastComments Angular preko NPM-a'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer zavisnost treba dodati u vašu tsconfig.json datoteku, na primer:

[inline-code-attrs-start title = 'Dodavanje fastcomments-typescript peer zavisnosti'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Zatim dodajte `FastCommentsModule` u vašu aplikaciju:

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

## Korišćenje

Za početak, prosleđujemo konfiguracijski objekat za demo zakupca:

[inline-code-attrs-start title = 'Korišćenje - Inline konfiguracija'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Pošto konfiguracija može postati prilično složena, možemo proslediti referencu objekta:

[inline-code-attrs-start title = 'Korišćenje - Prosledi objekat za konfiguraciju'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Korišćenje - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Vidžet koristi detekciju promena, tako da će promena bilo kojih svojstava konfiguracijskog objekta uzrokovati ponovno učitavanje.

Konfiguraciju koju Angular komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovde</a>.
