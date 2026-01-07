Našo Angular knjižnico lahko najdete na NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">tukaj</a>.

FastComments Angular pripomoček za komentarje podpira vse enake funkcije kot različica VanillaJS — komentiranje v realnem času, SSO in tako naprej.

Potrebovali boste fastcomments-typescript, ki je peer odvisnost. Prosimo, zagotovite, da je vključen v vašo TypeScript kompilacijo.
V prihodnosti bo ta peer odvisnost premaknjena v @types/fastcomments, kar bo poenostavilo namestitev.

[inline-code-attrs-start title = 'FastComments Angular prek NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer odvisnost je treba dodati v vašo datoteko tsconfig.json, na primer:

[inline-code-attrs-start title = 'Dodajanje fastcomments-typescript peer odvisnosti'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Nato dodajte `FastCommentsModule` v vašo aplikacijo:

[inline-code-attrs-start title = 'Dodajte modul v vašo aplikacijo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Uporaba

Za začetek posredujemo konfiguracijski objekt za demo najemnika:

[inline-code-attrs-start title = 'Uporaba - Inline konfiguracija'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Ker lahko konfiguracija postane precej zapletena, lahko posredujemo referenco objekta:

[inline-code-attrs-start title = 'Uporaba - Posreduj objekt za konfiguracijo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Uporaba - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Pripomoček uporablja zaznavanje sprememb, zato bo sprememba katerihkoli lastnosti konfiguracijskega objekta povzročila ponovno nalaganje.

Konfiguracijo, ki jo podpira Angular komponenta, lahko najdete <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tukaj</a>.
