Sie finden unsere Angular-Bibliothek auf NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">hier</a>.

Das FastComments Angular-Kommentar-Widget unterstützt alle Funktionen der VanillaJS-Version - Live-Kommentare, SSO und mehr.

Sie benötigen fastcomments-typescript als Peer-Abhängigkeit. Bitte stellen Sie sicher, dass diese in Ihrer TypeScript-Kompilierung enthalten ist.
In Zukunft wird diese Peer-Abhängigkeit nach @types/fastcomments verschoben, was diese Installation vereinfachen wird.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Die Peer-Abhängigkeit sollte in Ihrer tsconfig.json-Datei hinzugefügt werden, zum Beispiel:

[inline-code-attrs-start title = 'Adding fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Fügen Sie dann das `FastCommentsModule` zu Ihrer Anwendung hinzu:

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

## Verwendung

Um zu beginnen, übergeben wir ein Konfigurationsobjekt für den Demo-Mandanten:

[inline-code-attrs-start title = 'Usage - Inline Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Da die Konfiguration recht komplex werden kann, können wir eine Objektreferenz übergeben:

[inline-code-attrs-start title = 'Usage - Pass Object for Configuration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Usage - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Das Widget verwendet Änderungserkennung, sodass das Ändern von Eigenschaften des Konfigurationsobjekts ein Neuladen verursacht.

Die vom Angular-Komponenten unterstützte Konfiguration finden Sie <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.
