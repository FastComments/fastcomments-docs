Για να δείτε κάθε widget και flow να τρέχει τοπικά ενάντια στο δημόσιο ενοικιαστή `demo`, κλωνοποιήστε το αποθετήριο και τρέξτε:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Τα παραδείγματα συνδέονται με τη βιβλιοθήκη μέσω `file:../..`, έτσι το βήμα `npm run build` στη ρίζα χρειάζεται μία φορά για να παραχθεί το `dist/`.

Κάθε widget/flow έχει τη δική του προβολή στο `examples/example-showcase/src/views/` που μπορείτε να αντιγράψετε απευθείας στην δική σας εφαρμογή React.