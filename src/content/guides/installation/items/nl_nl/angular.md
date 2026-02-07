Als u reacties wilt toevoegen aan een met Angular gebouwde site, vindt u onze Angular-bibliotheek op NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">hier</a>.

De FastComments Angular-commentaar-widget ondersteunt alle functies van de VanillaJS-versie - live reacties, sso, enzovoort.

U heeft fastcomments-typescript nodig, dat een peer-dependency is. Zorg ervoor dat dit is opgenomen in uw TypeScript-compilatie.
In de toekomst wordt deze peer-dependency verplaatst naar @types/fastcomments, wat deze installatie zal vereenvoudigen.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

De peer-dependency moet worden toegevoegd aan uw tsconfig.json-bestand, bijvoorbeeld:

[inline-code-attrs-start title = 'Toevoegen van fastcomments-typescript als peer-dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Voeg daarna de `FastCommentsModule` toe aan uw applicatie:

[inline-code-attrs-start title = 'Module toevoegen aan uw app'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Om te beginnen geven we een config-object door voor de demo-tenant:

[inline-code-attrs-start title = 'Gebruik - Inlineconfiguratie'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Aangezien de configuratie behoorlijk ingewikkeld kan worden, kunnen we een objectreferentie doorgeven:

[inline-code-attrs-start title = 'Gebruik - Object doorgeven voor configuratie'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Gebruik - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

De widget gebruikt change detection, dus als u eigenschappen van het configuratieobject wijzigt, wordt het opnieuw geladen.

U kunt de configuratie die de Angular-component ondersteunt vinden <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.