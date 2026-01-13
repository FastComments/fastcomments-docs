U kunt onze Angular-bibliotheek op NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">hier</a> vinden.

De FastComments Angular reactiewidget ondersteunt alle dezelfde functies als de VanillaJS-versie - live reacties, SSO, enzovoort.

U hebt fastcomments-typescript nodig, wat een peer dependency is. Zorg ervoor dat dit is opgenomen in uw TypeScript-compilatie.
In de toekomst zal deze peer dependency worden verplaatst naar @types/fastcomments, wat deze installatie zal vereenvoudigen.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

De peer dependency moet worden toegevoegd in uw tsconfig.json-bestand, bijvoorbeeld:

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Voeg vervolgens de `FastCommentsModule` toe aan uw applicatie:

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

## Gebruik

Om te beginnen geven we een configuratieobject door voor de demo-tenant:

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Omdat de configuratie behoorlijk ingewikkeld kan worden, kunnen we een objectreferentie doorgeven:

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

De widget gebruikt wijzigingsdetectie, dus het wijzigen van eigenschappen van het configuratieobject zal ervoor zorgen dat het opnieuw wordt geladen.

U kunt de configuratie die de Angular-component ondersteunt <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a> vinden.
