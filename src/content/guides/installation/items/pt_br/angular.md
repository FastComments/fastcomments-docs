Você pode encontrar nossa biblioteca Angular no NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">aqui</a>.

O widget de comentários FastComments para Angular suporta todos os mesmos recursos da versão VanillaJS - comentários ao vivo, SSO e mais.

Você precisará do fastcomments-typescript, que é uma dependência peer. Por favor, certifique-se de que esteja incluído na sua compilação TypeScript.
No futuro, essa dependência peer será movida para @types/fastcomments, o que simplificará esta instalação.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

A dependência peer deve ser adicionada no seu arquivo tsconfig.json, por exemplo:

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Então, adicione o `FastCommentsModule` à sua aplicação:

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

Para começar, passamos um objeto de configuração para o tenant de demonstração:

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Como a configuração pode ficar bem complicada, podemos passar uma referência de objeto:

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

O widget usa detecção de mudanças, então alterar qualquer propriedade do objeto de configuração fará com que ele seja recarregado.

Você pode encontrar a configuração que o componente Angular suporta <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aqui</a>.
