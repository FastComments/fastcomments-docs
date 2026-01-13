Στο FastComments αποκαλούμε το αναγνωριστικό του άρθρου ή της σελίδας στην οποία συνδέονται τα σχόλια, URL ID, καθώς μπορεί να είναι είτε URL είτε ID.
Ορίστε το URL ID με τον ακόλουθο τρόπο. Το component παρακολουθεί αλλαγές στο αντικείμενο config και θα επαναφορτωθεί, οπότε μπορείτε να ενημερώσετε το URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Περιοχή Λογαριασμού (ΠΡΟΣΟΧΗ: ΠΕΛΑΤΕΣ ΕΕ)

Εάν ο λογαριασμός σας βρίσκεται στην ΕΕ, ορίστε `region = 'eu'` στη διαμόρφωση του widget, για παράδειγμα:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Διαφορετικά, δεν χρειάζεται να ορίσετε `region`.