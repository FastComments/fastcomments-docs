Za dodavanje komentara na web-stranicu izgrađenu s Angularom, našu Angular biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">ovdje</a>.

FastComments Angular widget za komentare podržava sve iste značajke kao i VanillaJS verzija - uživo komentiranje, SSO i slično.

Trebat će vam fastcomments-typescript, koji je peer ovisnost. Molimo osigurajte da je to uključeno u vašu TypeScript kompilaciju.
U budućnosti će ta peer ovisnost biti premještena u @types/fastcomments čime će se pojednostaviti ova instalacija.

[inline-code-attrs-start title = 'FastComments Angular putem NPM-a'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer ovisnost trebate dodati u svoju datoteku tsconfig.json, na primjer:

[inline-code-attrs-start title = 'Dodavanje peer ovisnosti fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Zatim dodajte `FastCommentsModule` u svoju aplikaciju:

[inline-code-attrs-start title = 'Dodajte modul u svoju aplikaciju'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Za početak, prosljeđujemo objekt konfiguracije za demo tenant:

[inline-code-attrs-start title = 'Korištenje - Inline konfiguracija'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Budući da konfiguracija može postati prilično složena, možemo proslijediti referencu na objekt:

[inline-code-attrs-start title = 'Korištenje - Proslijedite objekt za konfiguraciju'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Korištenje - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widget koristi detekciju promjena, tako da će promjena bilo koje svojstva objekta konfiguracije uzrokovati njegovo ponovno učitavanje.

Konfiguraciju koju Angular komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.

---