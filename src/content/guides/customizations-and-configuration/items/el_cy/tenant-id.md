[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Μπορεί να παρατηρήσετε ότι το widget σχολίων μπορεί να χρησιμοποιηθεί με Tenant ID "demo", για παράδειγμα:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Αυτό προορίζεται μόνο για να δοκιμάσετε και να πειραματιστείτε με το widget σχολίων. Σε παραγωγή, θα περάσετε το Tenant ID σας ως εξής:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Το Tenant ID σας μπορεί να βρεθεί ήδη εφαρμοσμένο στο widget σχολίων στο <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">απόσπασμα κώδικα στον λογαριασμό σας</a>.

Μπορείτε επίσης να βρείτε το Tenant ID σας και να διαχειριστείτε τα κλειδιά API σας [στη σελίδα διαπιστευτηρίων API](https://fastcomments.com/auth/my-account/api-secret).

Από αυτό το σημείο και μετά, αν είστε συνδεδεμένοι στο FastComments, τα παραδείγματα κώδικα θα χρησιμοποιούν το πραγματικό Tenant ID σας (αν είστε συνδεδεμένοι στο https://fastcomments.com).