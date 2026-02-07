Para añadir comentarios a un sitio construido con Angular, puede encontrar nuestra biblioteca de Angular en NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">aquí</a>.

El widget de comentarios FastComments para Angular admite todas las mismas funciones que el de VanillaJS: comentarios en tiempo real, sso, y demás.

Necesitará fastcomments-typescript, que es una dependencia peer. Asegúrese de que esto esté incluido en la compilación de TypeScript.
En el futuro, esta dependencia peer se moverá a @types/fastcomments, lo que simplificará esta instalación.

[inline-code-attrs-start title = 'FastComments Angular vía NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

La dependencia peer debe añadirse en su archivo tsconfig.json, por ejemplo:

[inline-code-attrs-start title = 'Agregar la dependencia peer fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Luego, agregue el `FastCommentsModule` a su aplicación:

[inline-code-attrs-start title = 'Agregar el módulo a su aplicación'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Para comenzar, pasamos un objeto de configuración para el tenant de demostración:

[inline-code-attrs-start title = 'Uso - Configuración en línea'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Dado que la configuración puede volverse bastante complicada, podemos pasar una referencia a un objeto:

[inline-code-attrs-start title = 'Uso - Pasar objeto para la configuración'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Uso - UE'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

El widget utiliza la detección de cambios, por lo que modificar cualquier propiedad del objeto de configuración provocará que se recargue.

Puede encontrar la configuración que admite el componente Angular <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aquí</a>.

---