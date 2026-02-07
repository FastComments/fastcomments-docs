Para adicionar comentários a um site construído com Angular, você pode encontrar nossa biblioteca Angular no NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">aqui</a>.

O widget de comentários FastComments para Angular oferece todos os mesmos recursos do VanillaJS — comentários em tempo real, SSO, e assim por diante.

Você precisará de fastcomments-typescript, que é uma dependência peer. Por favor, certifique-se de que isso esteja incluído na sua compilação TypeScript.
No futuro, essa dependência peer será movida para @types/fastcomments, o que simplificará essa instalação.

[inline-code-attrs-start title = 'FastComments para Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

A dependência peer deve ser adicionada no seu arquivo tsconfig.json, por exemplo:

[inline-code-attrs-start title = 'Adicionando a dependência peer fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Então, adicione o `FastCommentsModule` à sua aplicação:

[inline-code-attrs-start title = 'Adicione o módulo à sua aplicação'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Para começar, passamos um objeto de configuração para o tenant de demonstração:

[inline-code-attrs-start title = 'Uso - Configuração inline'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Como a configuração pode ficar bastante complicada, podemos passar uma referência de objeto:

[inline-code-attrs-start title = 'Uso - Passar objeto para configuração'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Uso - UE'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

O widget usa detecção de mudanças, então alterar qualquer propriedade do objeto de configuração fará com que ele seja recarregado.

Você pode encontrar a configuração que o componente Angular suporta <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aqui</a>.