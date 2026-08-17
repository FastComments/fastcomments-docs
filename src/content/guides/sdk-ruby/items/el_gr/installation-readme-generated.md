Προσθέστε αυτή τη γραμμή στο Gemfile της εφαρμογής σας:

```ruby
gem 'fastcomments'
```

Και στη συνέχεια εκτελέστε:

```bash
bundle install
```

Ή εγκαταστήστε το μόνοι σας ως:

```bash
gem install fastcomments
```

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τον παραγόμενο πελάτη API και τα εργαλεία SSO για να διευκολύνει τη χρήση του API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Δημόσια vs Ασφαλισμένα API

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, και η `PublicApi` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από έναν φυλλομετρητή/συσκευή κινητής τηλεφωνίας κ.λπ. χωρίς έλεγχο ταυτότητας. Η `ModerationApi` περιέχει τις μεθόδους που τροφοδοτούν τον πίνακα ελέγχου του συντονιστή.

Η `ModerationApi` παρέχει μια εκτενή σειρά ζωντανών και γρήγορων API συντονισμού. Κάθε μέθοδος της `ModerationApi` δέχεται μια παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή μέσω cookie συνεδρίας FastComments.com.