Pour ajouter des commentaires à un site construit avec Angular, vous pouvez trouver notre bibliothèque Angular sur NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">ici</a>.

Le widget de commentaires FastComments pour Angular prend en charge toutes les mêmes fonctionnalités que celui en VanillaJS - live commenting, sso, et ainsi de suite.

Vous aurez besoin de fastcomments-typescript, qui est une dépendance peer. Veuillez vous assurer qu'elle est incluse dans votre compilation TypeScript.
À l'avenir, cette dépendance peer sera déplacée vers @types/fastcomments ce qui simplifiera cette installation.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

La dépendance peer doit être ajoutée à votre fichier tsconfig.json, par exemple :

[inline-code-attrs-start title = 'Ajout de la dépendance peer fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Ensuite, ajoutez le `FastCommentsModule` à votre application :

[inline-code-attrs-start title = 'Ajouter le module à votre application'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Pour commencer, nous passons un objet de configuration pour le tenant de démonstration :

[inline-code-attrs-start title = 'Utilisation - Configuration inline'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Étant donné que la configuration peut devenir assez compliquée, nous pouvons passer une référence d'objet :

[inline-code-attrs-start title = 'Utilisation - Passer un objet pour la configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Utilisation - UE'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Le widget utilise la détection des modifications, donc la modification de n'importe quelle propriété de l'objet de configuration provoquera son rechargement.

Vous pouvez trouver la configuration prise en charge par le composant Angular <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ici</a>.

---