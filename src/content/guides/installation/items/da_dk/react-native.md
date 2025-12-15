Du kan finde vores React Native-bibliotek på NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">her</a>.

FastComments React Native kommentar-widget understøtter alle de samme funktioner som VanillaJS-versionen - live kommentering, SSO og så videre.

[inline-code-attrs-start title = 'FastComments React Native via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]

[inline-code-attrs-start title = 'FastComments React Native via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfigurationen specificeres lidt anderledes sammenlignet med `fastcomments-react`-biblioteket:

[inline-code-attrs-start title = 'React Native eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

const tenantId = 'demo';
const pageId = 'native-test';
const config = { tenantId: tenantId, urlId: pageId };

return (<FastCommentsCommentWidget config={config}/>);
[inline-code-end]

Du kan finde konfigurationen, som React Native-komponenten understøtter <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">her</a>.
