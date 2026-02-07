Aby dodać komentarze do strony zbudowanej w Angularze, naszą bibliotekę Angular można znaleźć na NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">tutaj</a>.

Widżet komentujący FastComments dla Angular obsługuje wszystkie te same funkcje co wersja VanillaJS — komentowanie na żywo, SSO i tak dalej.

Będziesz potrzebować fastcomments-typescript, które jest zależnością peer. Upewnij się, że jest ono uwzględnione w kompilacji TypeScript.
W przyszłości ta zależność zostanie przeniesiona do @types/fastcomments, co uprości instalację.

[inline-code-attrs-start title = 'FastComments Angular przez NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Zależność peer powinna zostać dodana do pliku tsconfig.json, na przykład:

[inline-code-attrs-start title = 'Dodawanie zależności peer fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Następnie dodaj `FastCommentsModule` do swojej aplikacji:

[inline-code-attrs-start title = 'Dodaj moduł do aplikacji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Użycie

Aby zacząć, przekazujemy obiekt konfiguracyjny dla najemcy demo:

[inline-code-attrs-start title = 'Użycie - konfiguracja inline'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Ponieważ konfiguracja może być dość skomplikowana, możemy przekazać referencję do obiektu:

[inline-code-attrs-start title = 'Użycie - przekazywanie obiektu konfiguracji'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Użycie - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Widżet używa wykrywania zmian, więc zmiana dowolnej właściwości obiektu konfiguracyjnego spowoduje jego ponowne załadowanie.

Konfigurację wspieraną przez komponent Angular można znaleźć <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.

---