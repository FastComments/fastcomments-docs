Puede encontrar nuestra biblioteca Angular en NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">aquí</a>.

El widget de comentarios FastComments para Angular admite todas las mismas funciones que la versión VanillaJS: comentarios en vivo, SSO y más.

Necesitará fastcomments-typescript, que es una dependencia de pares. Asegúrese de que esté incluida en su compilación de TypeScript.
En el futuro, esta dependencia de pares se moverá a @types/fastcomments, lo que simplificará esta instalación.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

La dependencia de pares debe agregarse en su archivo tsconfig.json, por ejemplo:

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Luego, agregue el `FastCommentsModule` a su aplicación:

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

## Uso

Para comenzar, pasamos un objeto de configuración para el inquilino de demostración:

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Dado que la configuración puede volverse bastante complicada, podemos pasar una referencia de objeto:

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

El widget utiliza detección de cambios, por lo que cambiar cualquier propiedad del objeto de configuración hará que se recargue.

Puede encontrar la configuración que admite el componente Angular <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aquí</a>.
