For at tilføje kommentarer til et websted bygget med Angular, kan du finde vores Angular-bibliotek på NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">her</a>.

FastComments Angular-kommenteringswidget'en understøtter alle de samme funktioner som VanillaJS-versionen - live-kommentering, sso, og så videre.

Du skal bruge fastcomments-typescript, som er en peer-afhængighed. Sørg venligst for, at dette er inkluderet i din TypeScript-kompilering.
I fremtiden vil denne peer-afhængighed blive flyttet til @types/fastcomments, hvilket vil forenkle denne installation.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Peer-afhængigheden skal tilføjes i din tsconfig.json-fil, for eksempel:

[inline-code-attrs-start title = 'Tilføjelse af fastcomments-typescript peer-afhængighed'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Tilføj derefter `FastCommentsModule` til din applikation:

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

For at komme i gang angiver vi et konfigurationsobjekt for demo-tenant:

[inline-code-attrs-start title = 'Brug - Inline-konfiguration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Da konfigurationen kan blive ret kompliceret, kan vi videregive en objektreference:

[inline-code-attrs-start title = 'Brug - Videregiv objekt til konfiguration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Brug - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widget'en bruger change detection, så ændring af nogen egenskaber i konfigurationsobjektet vil få den til at blive genindlæst.

Du kan finde den konfiguration, som Angular-komponenten understøtter <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">her</a>.

---