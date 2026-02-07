Um Kommentare auf einer mit Angular erstellten Website hinzuzufügen, finden Sie unsere Angular-Bibliothek auf NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">hier</a>.

Das FastComments Angular-Kommentarmodul unterstützt alle dieselben Funktionen wie das VanillaJS-Widget - Live-Kommentare, SSO und so weiter.

Sie benötigen fastcomments-typescript, das als Peer-Abhängigkeit fungiert. Bitte stellen Sie sicher, dass dies in Ihre TypeScript-Kompilierung aufgenommen ist.
Zukünftig wird diese Peer-Abhängigkeit zu @types/fastcomments verschoben, was diese Installation vereinfachen wird.

[inline-code-attrs-start title = 'FastComments Angular über NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Die Peer-Abhängigkeit sollte in Ihrer tsconfig.json-Datei hinzugefügt werden, zum Beispiel:

[inline-code-attrs-start title = 'Hinzufügen der fastcomments-typescript Peer-Abhängigkeit'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Fügen Sie dann das `FastCommentsModule` zu Ihrer Anwendung hinzu:

[inline-code-attrs-start title = 'Modul zu Ihrer App hinzufügen'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Um zu beginnen, übergeben wir ein Konfigurationsobjekt für den Demo-Tenant:

[inline-code-attrs-start title = 'Verwendung - Inline-Konfiguration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Da die Konfiguration ziemlich komplex werden kann, können wir eine Objektreferenz übergeben:

[inline-code-attrs-start title = 'Verwendung - Objekt zur Konfiguration übergeben'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Verwendung - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Das Widget verwendet Change Detection, sodass das Ändern von Eigenschaften des Konfigurationsobjekts dazu führt, dass es neu geladen wird.

Die Konfiguration, die die Angular-Komponente unterstützt, finden Sie <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.

---