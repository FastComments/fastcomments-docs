---
Per aggiungere commenti a un sito realizzato con Angular, puoi trovare la nostra libreria Angular su NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">qui</a>.

Il widget di commenti FastComments per Angular supporta tutte le stesse funzionalità di quello VanillaJS - commenti in tempo reale, SSO, e così via.

Avrai bisogno di fastcomments-typescript, che è una dipendenza peer. Assicurati che sia inclusa nella compilazione TypeScript.
In futuro, questa dipendenza peer verrà spostata su @types/fastcomments, il che semplificherà questa installazione.

[inline-code-attrs-start title = 'FastComments Angular tramite NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

La dipendenza peer dovrebbe essere aggiunta nel tuo file tsconfig.json, per esempio:

[inline-code-attrs-start title = 'Aggiunta della dipendenza peer fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Quindi, aggiungi il `FastCommentsModule` alla tua applicazione:

[inline-code-attrs-start title = 'Aggiungi il modulo alla tua applicazione'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Utilizzo - Configurazione inline'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Poiché la configurazione può diventare piuttosto complicata, possiamo passare un riferimento a un oggetto:

[inline-code-attrs-start title = 'Utilizzo - Passare un oggetto per la configurazione'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Utilizzo - UE'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Il widget utilizza il rilevamento delle modifiche, quindi modificare qualsiasi proprietà dell'oggetto di configurazione ne causerà il ricaricamento.

Puoi trovare la configurazione supportata dal componente Angular <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">qui</a>.

---