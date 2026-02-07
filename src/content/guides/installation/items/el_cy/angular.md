Για να προσθέσετε σχόλια σε έναν ιστότοπο φτιαγμένο με Angular, μπορείτε να βρείτε τη βιβλιοθήκη μας για Angular στο NPM <a href="https://www.npmjs.com/package/ngx-fastcomments" target="_blank">εδώ</a>.

Το FastComments Angular commenting widget υποστηρίζει όλες τις ίδιες δυνατότητες με το VanillaJS — ζωντανά σχόλια, SSO, κ.λπ.

Θα χρειαστείτε το fastcomments-typescript, που είναι ένα peer dependency. Παρακαλώ βεβαιωθείτε ότι αυτό είναι συμπεριλαμβανόμενο στην μεταγλώττιση TypeScript σας.
Στο μέλλον, αυτό το peer dependency θα μεταφερθεί στο @types/fastcomments, κάτι που θα απλοποιήσει αυτήν την εγκατάσταση.

[inline-code-attrs-start title = 'FastComments Angular μέσω NPM'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  npm install fastcomments-typescript --save
  npm install ngx-fastcomments --save
[inline-code-end]

Το peer dependency θα πρέπει να προστεθεί στο αρχείο tsconfig.json σας, για παράδειγμα:

[inline-code-attrs-start title = 'Προσθήκη του peer dependency fastcomments-typescript'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
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

Για να ξεκινήσετε, περνάμε ένα αντικείμενο ρύθμισης για τον demo tenant:

[inline-code-attrs-start title = 'Χρήση - Ενσωματωμένη διαμόρφωση'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo' }"></lib-fastcomments>
[inline-code-end]

Εφόσον η διαμόρφωση μπορεί να γίνει αρκετά περίπλοκη, μπορούμε να περάσουμε μια αναφορά σε αντικείμενο:

[inline-code-attrs-start title = 'Χρήση - Παράδοση αντικειμένου για διαμόρφωση'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="fastcommentsConfig"></lib-fastcomments>
[inline-code-end]

[inline-code-attrs-start title = 'Χρήση - ΕΕ'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<lib-fastcomments [config]="{ tenantId: 'demo', region: 'eu' }"></lib-fastcomments>
[inline-code-end]

Το widget χρησιμοποιεί ανίχνευση αλλαγών, οπότε η αλλαγή οποιασδήποτε ιδιότητας του αντικειμένου διαμόρφωσης θα το ξαναφορτώσει.

Μπορείτε να βρείτε τη διαμόρφωση που υποστηρίζει το Angular component <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">εδώ</a>.