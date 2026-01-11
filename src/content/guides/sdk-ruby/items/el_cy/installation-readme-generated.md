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

### Περιεχόμενα Βιβλιοθήκης

Αυτή η βιβλιοθήκη περιέχει τον παραγόμενο API client και τα βοηθητικά εργαλεία SSO που κάνουν την εργασία με το API πιο εύκολη.

- [Τεκμηρίωση βιβλιοθήκης API Client](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Δημόσια έναντι Ασφαλών APIs

Για τον API client, υπάρχουν δύο κλάσεις, `DefaultApi` και `PublicApi`. Η `DefaultApi` περιέχει μεθόδους που απαιτούν το API key σας, και η `PublicApi` περιέχει κλήσεις API που μπορούν να γίνουν απευθείας από ένα πρόγραμμα περιήγησης/κινητή συσκευή/κ.ά. χωρίς πιστοποίηση.