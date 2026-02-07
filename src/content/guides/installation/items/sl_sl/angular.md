Za dodajanje komentarjev na spletno mesto, izdelano z Angular, lahko najdete našo Angular knjižnico na NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">tukaj</a>.

Komentarni pripomoček FastComments za Angular podpira vse iste funkcije kot različica VanillaJS - komentiranje v živo, SSO itd.

Potrebovali boste fastcomments-typescript, ki je peer-odvisnost. Prosimo, zagotovite, da je vključena v vašo TypeScript kompilacijo.
V prihodnosti bo ta peer-odvisnost premaknjena v @types/fastcomments, kar bo poenostavilo to namestitev.

[inline-code-attrs-start title = 'FastComments za Angular prek NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

To peer-odvisnost dodajte v datoteko tsconfig.json, na primer:

[inline-code-attrs-start title = 'Dodajanje peer-odvisnosti fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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

## Usage

Za začetek podamo konfiguracijski objekt za demo najemnika:

[inline-code-attrs-start title = 'Uporaba - vrstična konfiguracija'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Ker je konfiguracija lahko precej zapletena, lahko namesto tega podamo referenco na objekt:

[inline-code-attrs-start title = 'Uporaba - posredovanje objekta za konfiguracijo'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Uporaba - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Pripomoček uporablja zaznavanje sprememb, zato bo sprememba katerekoli lastnosti konfiguracijskega objekta povzročila njegovo ponovno nalaganje.

Konfiguracijo, ki jo podpira Angular komponenta, najdete <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tukaj</a>.

---