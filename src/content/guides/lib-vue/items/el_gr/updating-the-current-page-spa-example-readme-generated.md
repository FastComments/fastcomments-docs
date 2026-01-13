Στο FastComments, το id του άρθρου, ή της σελίδας με την οποία συνδέονται τα σχόλια, το ονομάζουμε URL ID καθώς μπορεί να είναι ένα url ή ένα ID.
Ορίστε το URL ID με τον ακόλουθο τρόπο. Το component παρακολουθεί αλλαγές στο αντικείμενο config και θα ξαναφορτώσει, οπότε μπορείτε απλώς να ενημερώσετε τις ρυθμίσεις "url" και "urlId".

Δείτε ένα πλήρες λειτουργικό παράδειγμα [εδώ](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Εκτελέστε το παράδειγμα σελιδοποίησης μέσω:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Περιοχή λογαριασμού (ΠΡΟΣΟΧΗ: Πελάτες ΕΕ)

Εάν ο λογαριασμός σας βρίσκεται στην ΕΕ, ορίστε `region = 'eu'` στη διαμόρφωση του widget, για παράδειγμα:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Διαφορετικά, δεν χρειάζεται να ορίσετε `region`.