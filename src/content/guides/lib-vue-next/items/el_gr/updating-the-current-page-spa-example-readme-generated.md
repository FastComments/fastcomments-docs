Στο FastComments αποκαλούμε το id του άρθρου, ή τη σελίδα στην οποία συνδέονται τα σχόλια, το URL ID καθώς μπορεί να είναι ένα url ή ένα ID.
Ορίστε το URL ID με τον ακόλουθο τρόπο. Το component παρακολουθεί αλλαγές στο αντικείμενο config, και θα ξαναφορτώσει, οπότε μπορείτε να ενημερώσετε το URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Account Region (ATTENTION: EU Customers)

Εάν ο λογαριασμός σας βρίσκεται στην ΕΕ, ορίστε `region = 'eu'` στη διαμόρφωση του widget, για παράδειγμα:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Διαφορετικά, δεν χρειάζεται να ορίσετε `region`.