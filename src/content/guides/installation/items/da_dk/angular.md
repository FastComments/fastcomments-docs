Du kan finde vores Angular-bibliotek på NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">her</a>.

FastComments Angular kommentar-widget understøtter alle de samme funktioner som VanillaJS-versionen - live kommentering, SSO og så videre.

[inline-code-attrs-start title = 'FastComments Angular via NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install fastcomments-typescript --save
npm install ngx-fastcomments --save
[inline-code-end]

## Brug

[inline-code-attrs-start title = 'Brug - Inline konfiguration'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Du kan finde konfigurationen, som Angular-komponenten understøtter <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">her</a>.
