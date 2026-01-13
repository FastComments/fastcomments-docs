Vous pouvez trouver notre bibliothèque Angular sur NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">ici</a>.

Le widget de commentaires FastComments pour Angular prend en charge toutes les mêmes fonctionnalités que la version VanillaJS - commentaires en direct, SSO, et plus encore.

Vous aurez besoin de fastcomments-typescript, qui est une dépendance de pair. Veuillez vous assurer qu'elle est incluse dans votre compilation TypeScript.
À l'avenir, cette dépendance de pair sera déplacée vers @types/fastcomments, ce qui simplifiera cette installation.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

La dépendance de pair doit être ajoutée dans votre fichier tsconfig.json, par exemple :

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Ensuite, ajoutez le `FastCommentsModule` à votre application :

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

## Utilisation

Pour commencer, nous passons un objet de configuration pour le locataire de démonstration :

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Puisque la configuration peut devenir assez complexe, nous pouvons passer une référence d'objet :

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Le widget utilise la détection de changements, donc modifier n'importe quelle propriété de l'objet de configuration entraînera son rechargement.

Vous pouvez trouver la configuration prise en charge par le composant Angular <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ici</a>.
