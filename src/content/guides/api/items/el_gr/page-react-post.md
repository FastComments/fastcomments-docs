[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Προσθέτει μια αντίδραση σε μια σελίδα ως ο τρέχων χρήστης. Ένας χρήστης μπορεί να προσθέσει κάθε αναγνωριστικό αντίδρασης μία φορά: η προσθήκη ξανά επιτυγχάνει με τον κωδικό `already-reacted` και δεν αλλάζει το πλήθος. Ένας χρήστης μπορεί να προσθέσει πολλές διαφορετικές αντιδράσεις στην ίδια σελίδα.

Τα αναγνωριστικά αντιδράσεων επιλέγονται από εσάς και μπορεί να έχουν έως 36 χαρακτήρες. Η σελίδα δημιουργείται αν δεν υπάρχει ακόμη. Περνάτε το `title` για να ορίσετε ή να ενημερώσετε τον τίτλο της σελίδας.

[inline-code-attrs-start title = 'Παράδειγμα cURL Αντίδρασης Σελίδας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αντίδρασης Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** The reaction id, up to 36 characters. **/
    id: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Αντίδρασης Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' when the user had already added this reaction. 'react-id-too-long' (HTTP 422) when the id is over 36 characters. Otherwise included on failure. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]