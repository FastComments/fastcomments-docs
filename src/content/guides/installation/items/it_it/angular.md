Puoi trovare la nostra libreria Angular su NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">qui</a>.

Il widget dei commenti FastComments per Angular supporta tutte le stesse funzionalità della versione VanillaJS - commenti in tempo reale, SSO e altro.

Avrai bisogno di fastcomments-typescript, che è una dipendenza peer. Assicurati che sia inclusa nella tua compilazione TypeScript.
In futuro, questa dipendenza peer verrà spostata in @types/fastcomments che semplificherà questa installazione.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

La dipendenza peer deve essere aggiunta nel tuo file tsconfig.json, per esempio:

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Quindi, aggiungi il `FastCommentsModule` alla tua applicazione:

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

## Utilizzo

Per iniziare, passiamo un oggetto di configurazione per il tenant demo:

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Poiché la configurazione può diventare piuttosto complicata, possiamo passare un riferimento a un oggetto:

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Il widget utilizza il rilevamento delle modifiche, quindi cambiare qualsiasi proprietà dell'oggetto di configurazione causerà il suo ricaricamento.

Puoi trovare la configurazione supportata dal componente Angular <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">qui</a>.
