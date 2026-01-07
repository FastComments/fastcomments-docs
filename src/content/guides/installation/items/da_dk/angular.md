Du kan finde vores Angular-bibliotek på NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">her</a>.

FastComments Angular kommentar-widget understøtter alle de samme funktioner som VanillaJS-versionen - live kommentering, SSO og så videre.

Du skal bruge fastcomments-typescript, som er en peer-afhængighed. Sørg for at dette er inkluderet i din TypeScript-kompilering.
I fremtiden vil denne peer-afhængighed blive flyttet til @types/fastcomments, hvilket vil forenkle installationen.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer-afhængigheden skal tilføjes i din tsconfig.json-fil, for eksempel:

[inline-code-attrs-start title = 'Tilføj fastcomments-typescript peer-afhængighed'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Derefter tilføj `FastCommentsModule` til din applikation:

[inline-code-attrs-start title = 'Tilføj modulet til din app'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Brug

For at komme i gang sender vi et konfigurationsobjekt for demo-tenant:

[inline-code-attrs-start title = 'Brug - Inline konfiguration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Da konfigurationen kan blive ret kompliceret, kan vi sende en objektreference:

[inline-code-attrs-start title = 'Brug - Send objekt til konfiguration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Brug - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widget'en bruger ændringsdetektion, så ændring af egenskaber på konfigurationsobjektet vil få det til at genindlæses.

Du kan finde konfigurationen, som Angular-komponenten understøtter <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">her</a>.
