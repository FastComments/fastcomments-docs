Μπορείτε να βρείτε τη βιβλιοθήκη Angular μας στο NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">εδώ</a>.

Το FastComments Angular widget σχολίων υποστηρίζει όλες τις ίδιες δυνατότητες με την έκδοση VanillaJS - ζωντανά σχόλια, SSO και ούτω καθεξής.

Θα χρειαστείτε το fastcomments-typescript, που είναι peer dependency. Βεβαιωθείτε ότι περιλαμβάνεται στη μεταγλώττιση TypeScript σας.
Στο μέλλον, αυτό το peer dependency θα μετακινηθεί στο @types/fastcomments που θα απλοποιήσει την εγκατάσταση.

[inline-code-attrs-start title = 'FastComments Angular μέσω NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Το peer dependency πρέπει να προστεθεί στο αρχείο tsconfig.json σας, για παράδειγμα:

[inline-code-attrs-start title = 'Προσθήκη fastcomments-typescript peer dependency'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
"include": [
  "src/**/*.ts",
  "node_modules/fastcomments-typescript/src/index.ts"
],
[inline-code-end]

Στη συνέχεια, προσθέστε το `FastCommentsModule` στην εφαρμογή σας:

[inline-code-attrs-start title = 'Προσθέστε το Module στην εφαρμογή σας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

## Χρήση

Για να ξεκινήσουμε, περνάμε ένα αντικείμενο ρύθμισης για τον demo tenant:

[inline-code-attrs-start title = 'Χρήση - Inline διαμόρφωση'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Δεδομένου ότι η διαμόρφωση μπορεί να γίνει αρκετά περίπλοκη, μπορούμε να περάσουμε μια αναφορά αντικειμένου:

[inline-code-attrs-start title = 'Χρήση - Πέρασμα αντικειμένου για διαμόρφωση'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Χρήση - EU'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Το widget χρησιμοποιεί ανίχνευση αλλαγών, οπότε η αλλαγή οποιωνδήποτε ιδιοτήτων του αντικειμένου διαμόρφωσης θα προκαλέσει επαναφόρτωση.

Μπορείτε να βρείτε τη διαμόρφωση που υποστηρίζει το Angular component <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">εδώ</a>.
