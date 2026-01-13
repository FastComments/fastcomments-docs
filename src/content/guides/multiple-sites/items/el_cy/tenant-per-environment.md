Είναι συνηθισμένο να υπάρχει ένας sub tenant για κάθε περιβάλλον test ή dev στο FastComments. Κάθε tenant θα έχει τη δική του διαμόρφωση, δεδομένα, και API keys. Η διαμόρφωση, τα δεδομένα, και οι χρήστες δεν μπορούν να μοιραστούν ανάμεσα σε tenants.
Όλα είναι απομονωμένα. Ωστόσο, οι super admins του parent tenant μπορούν να impersonate χρήστες σε child tenants.

There are two approaches:

- The main tenant is for production, and sub-tenants are for test environments.
- The main tenant is simply for billing, and each sub-tenant is for prod, test, and so on.

Η πρώτη προσέγγιση είναι γενικά πιο εύκολη για τους χρήστες να την κατανοήσουν, αλλά αυτό μπορεί να εξαρτάται από την οργάνωσή σας.

Tenants can be created [εδώ](https://eu.fastcomments.com/auth/my-account/tenants) if you have the package. This is also where super admins would
impersonate users. Tenants can also be created via the API for more custom/automated setups.

Όποια προσέγγιση κι αν ακολουθήσετε, θα πρέπει να προσθέσετε τους moderators και τους χρήστες που θέλουν να βλέπουν τα production δεδομένα στο tenant "prod". Έτσι, για παράδειγμα, αν θέλετε να πάτε με την επιλογή B και να έχετε τον parent tenant για billing, και ένα sub tenant για το "prod", θα θελήσετε να προσθέσετε τον tenant, να μεταβείτε στον νέο tenant, και να προσθέσετε τους admin και moderator χρήστες για τον sub-tenant. 

Τέλος, για να διευκρινίσουμε, η σελίδα Moderate Comments θα είναι κενή με την επιλογή B για τον parent tenant.