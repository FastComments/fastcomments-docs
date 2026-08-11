FastComments ελέγχει τα αιτήματα προς τον λογαριασμό σας για να βεβαιωθεί ότι προέρχονται από τον ιστότοπό σας. Γι' αυτό χρειάζεται να γνωρίζουμε ποιος ιστότοπος ή ποιοι ιστότοποι θέλετε να εγκαταστήσετε το FastComments.

FastComments υποστηρίζει έλεγχο ταυτότητας μέσω domain, καθώς και subdomains.

Ας πάρουμε τον ιστότοπο `https://example.com`. Σε αυτήν την περίπτωση, "`example.com`" είναι το domain. `example.com` υποστηρίζει τόσο `example.com`, όσο και `www.example.com`. Θα ονομάσουμε το "www" το "subdomain".

Για Παράδειγμα:

- Για να επιτρέψετε μόνο `blog.example.com`:
  - Προσθέστε `blog.example.com` στα domains σας.
- Για να επιτρέψετε `www.example.com`, `somesite.example.com` και `example.com`:
  - Προσθέστε `example.com` στα domains σας.
  - Αυτό χρεώνεται ως **ένα domain** που συνδέεται με τον λογαριασμό σας.
- Τώρα μπορείτε να προσθέσετε wildcard subdomains, για παράδειγμα *myname.vercel.app.
  - Αυτό χρεώνεται ως **ένα domain** που συνδέεται με τον λογαριασμό σας.

Αν χρησιμοποιούσατε μια πλατφόρμα blogging και σας είχε δοθεί ένα subdomain, θα θέλατε να προσθέσετε το **πλήρες domain συμπεριλαμβανομένου του subdomain** στον λογαριασμό σας, για παράδειγμα: `cats.blogger.com`.

Μπορούμε να προσθέσουμε domains στον λογαριασμό μας επισκεπτόμενοι τη σελίδα `My Domains` και κάνοντας κλικ στο `Add a Domain` στο κάτω μέρος:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Σελίδα My Domains που εμφανίζει τα domains στον λογαριασμό, με το κουμπί Add a Domain στο κάτω μέρος'; title='Η σελίδα My Domains' app-screenshot-end]

Κατά τη διάρκεια της δοκιμαστικής περιόδου, **τα domains προστίθενται αυτόματα στον λογαριασμό σας** όταν τα αιτήματα προέρχονται από τα εν λόγω domains. Ωστόσο, μετά από αυτό το διάστημα πρέπει να προστεθούν ρητά για λόγους ασφαλείας. Θα πρέπει να λάβετε ένα email όταν συμβεί αυτή η αυτοματοποιημένη συμπεριφορά.

Δεν χρειάζεται να προσθέσετε το `localhost` για τοπική ανάπτυξη – επιτρέπεται εξ ορισμού.

#### Μέσω του API

Τα domains μπορούν επίσης να προστεθούν και να ρυθμιστούν [via the DomainConfigs API](/guide-api.html#domain-config-structure).