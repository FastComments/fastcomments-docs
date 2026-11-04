Στο FastComments ονομάζουμε το αναγνωριστικό του άρθρου, ή τη σελίδα στην οποία συνδέονται τα σχόλια, URL ID, καθώς μπορεί να είναι είτε URL είτε ID.
Ορίστε το URL ID με τον εξής τρόπο. Το component παρακολουθεί τις αλλαγές στο αντικείμενο config και θα επαναφορτωθεί, ώστε να μπορείτε να ενημερώσετε το URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Περιοχή Λογαριασμού (ΠΡΟΣΟΧΗ: Πελάτες ΕΕ)

Αν ο λογαριασμός σας βρίσκεται στην ΕΕ, ορίστε `region = 'eu'` στη διαμόρφωση του widget, για παράδειγμα:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Διαφορετικά, δεν χρειάζεται να ορίσετε το `region`.