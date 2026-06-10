#### Θέμα: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Θέμα: Default
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Ενσωματωμένος WYSIWYG Επεξεργαστής με Υποστήριξη Εικόνων!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Επεξεργαστής Πλούσιου Κειμένου

Αυτή η βιβλιοθήκη χρησιμοποιεί [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) για επεξεργασία πλούσιου κειμένου, προσφέροντας μια ισχυρή εμπειρία WYSIWYG. Ο ίδιος επεξεργαστής τροφοδοτεί iOS, Android και το web (μέσω `react-native-web`), οπότε ο συνθέτης συμπεριφέρεται ομοιόμορφα σε κάθε πλατφόρμα με μία υλοποίηση.

Το `react-native-enriched` απαιτεί το React Native New Architecture (Fabric) στο native, και έναν bundler που επιλύει τις συνθήκες εξαγωγών πακέτων (Metro με package exports / RN 0.72+). Η υποστήριξη για web είναι αυτή τη στιγμή πειραματική.

### Επιλογές Διαμόρφωσης

Αυτή η βιβλιοθήκη στοχεύει να υποστηρίξει όλες τις επιλογές διαμόρφωσης που ορίζονται στο [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), όπως και η υλοποίηση για το web.

### Έννοιες FastComments

Οι βασικές έννοιες που πρέπει να γνωρίζετε για να ξεκινήσετε είναι οι `tenantId` και `urlId`. Το `tenantId` είναι ο αναγνωριστικός της λογαριασμού σας στο FastComments.com. Το `urlId` είναι όπου θα συνδεθούν οι νήματα σχολίων. Αυτό μπορεί να είναι ένα URL σελίδας, ή ένα αναγνωριστικό προϊόντος, ένα αναγνωριστικό άρθρου κ.λπ.

### Ειδοποιήσεις Χρηστών

Το FastComments υποστηρίζει ειδοποιήσεις για [πολλά σενάρια](https://docs.fastcomments.com/guide-notifications.html). Οι ειδοποιήσεις είναι ρυθμιζόμενες,
μπορούν να απενεργοποιηθούν παγκοσμίως ή σε επίπεδο ειδοποίησης/σχολίου, και υποστηρίζουν εγγραφές σε επίπεδο σελίδας ώστε οι χρήστες να μπορούν να εγγραφούν σε νήματα μιας
συγκεκριμένης σελίδας ή άρθρου.

Για παράδειγμα, είναι δυνατό να χρησιμοποιήσετε Secure SSO για να αυθεντικοποιήσετε τον χρήστη και στη συνέχεια περιοδικά να ελέγχετε για μη αναγνωσμένες ειδοποιήσεις και να τις προωθείτε στον χρήστη.

Δείτε [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) για το πώς να λαμβάνετε και να μεταφράζετε τις μη αναγνωσμένες ειδοποιήσεις χρήστη.

### Περιηγητής GIF

Από προεπιλογή, δεν είναι ενεργοποιημένη η επιλογή εικόνας ή gif. Δείτε [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) για το πώς
να υποστηρίξετε μεταφορτώσεις εικόνων και gif. Υπάρχει ένας Περιηγητής GIF που ανωνυμοποιεί τις αναζητήσεις και τις εικόνες που παρέχονται σε αυτή τη βιβλιοθήκη, απλά πρέπει να τον χρησιμοποιήσετε.

### Απόδοση

Παρακαλώ ανοίξτε ένα ticket με ένα παράδειγμα για να αναπαραχθεί το ζήτημα, συμπεριλαμβανομένης της συσκευής που χρησιμοποιήθηκε, εάν εντοπίσετε προβλήματα απόδοσης. Η απόδοση είναι πρώτη προτεραιότητα
σε όλες τις βιβλιοθήκες FastComments.