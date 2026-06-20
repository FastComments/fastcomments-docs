Προσθέστε αυτή τη γραμμή στο Gemfile της εφαρμογής σας:

```ruby
gem 'fastcomments'
```

Και στη συνέχεια εκτελέστε:

```bash
bundle install
```

Ή εγκαταστήστε το χειροκίνητα ως:

```bash
gem install fastcomments
```

### Περιεχόμενα βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τον αυτόματα δημιουργημένο API client και τα βοηθητικά εργαλεία SSO για να διευκολύνουν την εργασία με το API.

- [Τεκμηρίωση βιβλιοθήκης πελάτη API](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Δημόσια και Προστατευμένα API

Για τον πελάτη API, υπάρχουν τρεις κλάσεις, `DefaultApi`, `PublicApi`, και `ModerationApi`. Το `DefaultApi` περιλαμβάνει μεθόδους που απαιτούν το API key σας, και το `PublicApi` περιλαμβάνει κλήσεις API που μπορούν να γίνουν απευθείας από ένα πρόγραμμα περιήγησης/κινητή συσκευή/κ.λπ. χωρίς έλεγχο ταυτότητας. Το `ModerationApi` περιλαμβάνει τις μεθόδους που τροφοδοτούν τον πίνακα ελέγχου των συντονιστών.

Το `ModerationApi` καλύπτει τη διαχείριση σχολίων (list, count, search, logs, export), ενέργειες μεσολάβησης (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), απαγορεύσεις (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), και διακριτικά & εμπιστοσύνη (award/remove badge, manual badges, get/set trust factor, user internal profile). Κάθε μέθοδος του `ModerationApi` δέχεται μια παράμετρο `sso` ώστε το αίτημα να μπορεί να γίνει εκ μέρους ενός συντονιστή που έχει πιστοποιηθεί μέσω SSO.