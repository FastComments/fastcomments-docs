Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Library Contents

Αυτή η βιβλιοθήκη περιέχει τον παραγόμενο πελάτη API και τα εργαλεία SSO για να διευκολύνουν τη χρήση του API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi` και `ModerationApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το κλειδί API σας, η `PublicApi` περιλαμβάνει κλήσεις API που μπορούν να γίνουν απευθείας από πρόγραμμα περιήγησης/συσκευή κινητής κ.λπ. χωρίς έλεγχο ταυτότητας. Η `ModerationApi` περιέχει τις μεθόδους που τροφοδοτούν τον πίνακα ελέγχου του συντονιστή.

Η `ModerationApi` προσφέρει ένα εκτενές σύνολο ζωντανών και γρήγορων API συντονισμού. Κάθε μέθοδος `ModerationApi` δέχεται ένα παράμετρο `sso` και μπορεί να πιστοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.