Našu Angular biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">ovdje</a>.

FastComments Angular widget za komentare podržava sve iste značajke kao VanillaJS verzija — komentiranje u stvarnom vremenu, SSO i tako dalje.

Trebat će vam fastcomments-typescript, koji je peer ovisnost. Molimo osigurajte da je uključen u vašu TypeScript kompilaciju.
U budućnosti će ova peer ovisnost biti premještena u @types/fastcomments što će pojednostaviti instalaciju.

[inline-code-attrs-start title = 'FastComments Angular putem NPM-a'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer ovisnost treba dodati u vašu tsconfig.json datoteku, na primjer:

[inline-code-attrs-start title = 'Dodavanje fastcomments-typescript peer ovisnosti'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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

## Korištenje

Za početak, prosljeđujemo konfiguracijski objekt za demo tenant:

[inline-code-attrs-start title = 'Korištenje - Inline konfiguracija'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Budući da konfiguracija može postati prilično složena, možemo proslijediti referencu objekta:

[inline-code-attrs-start title = 'Korištenje - Proslijedi objekt za konfiguraciju'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Korištenje - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widget koristi detekciju promjena, tako da će promjena bilo kojih svojstava konfiguracijskog objekta uzrokovati ponovno učitavanje.

Konfiguraciju koju Angular komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.
