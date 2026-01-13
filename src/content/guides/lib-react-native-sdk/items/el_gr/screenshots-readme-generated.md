#### Θέμα: Erebus
![Θέμα: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Θέμα: Προεπιλογή
![Θέμα: Προεπιλογή](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Ενσωματωμένος WYSIWYG επεξεργαστής με υποστήριξη εικόνων!
![Ενσωματωμένος WYSIWYG επεξεργαστής με υποστήριξη εικόνων](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Επεξεργαστής Πλούσιου Κειμένου

Αυτή η βιβλιοθήκη χρησιμοποιεί τον επεξεργαστή 10tap για λειτουργικότητα επεξεργασίας πλούσιου κειμένου, ο οποίος προσφέρει μια ισχυρή εμπειρία επεξεργασίας WYSIWYG.

### Επιλογές Διαμόρφωσης

Η βιβλιοθήκη στοχεύει να υποστηρίζει όλες τις επιλογές διαμόρφωσης που ορίζονται στο [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), όπως και η υλοποίηση για web.

### Έννοιες του FastComments

Οι κύριες έννοιες που πρέπει να γνωρίζετε για να ξεκινήσετε είναι `tenantId` και `urlId`. Το `tenantId` είναι ο αναγνωριστικός κωδικός του λογαριασμού σας στο FastComments.com. Το `urlId` είναι εκεί όπου θα συνδεθούν τα νήματα σχολίων. Αυτό μπορεί να είναι ένα URL σελίδας, ή ένα id προϊόντος, ένα id άρθρου κ.λπ.

### Ειδοποιήσεις Χρηστών

Το FastComments υποστηρίζει ειδοποιήσεις για [πολλά σενάρια](https://docs.fastcomments.com/guide-notifications.html). Οι ειδοποιήσεις είναι ρυθμιζόμενες,
μπορούν να απενεργοποιηθούν παγκοσμίως ή σε επίπεδο ειδοποίησης/σχολίου, και υποστηρίζουν εγγραφές σε επίπεδο σελίδας ώστε οι χρήστες να μπορούν να εγγραφούν σε νήματα μιας
συγκεκριμένης σελίδας ή άρθρου.

Για παράδειγμα, είναι δυνατή η χρήση Secure SSO για να πιστοποιηθεί ο χρήστης και στη συνέχεια περιοδικά να ελέγχετε για μη αναγνωσμένες ειδοποιήσεις και να τις προωθείτε στον χρήστη.

Δείτε [το παράδειγμα AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) για το πώς να λαμβάνετε και να μεταφράζετε τις μη αναγνωσμένες ειδοποιήσεις χρηστών.

### Περιηγητής GIF

Εξ ορισμού, δεν είναι ενεργοποιημένη η επιλογή εικόνας ή gif. Δείτε [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) για το πώς
να υποστηρίξετε μεταφορτώσεις εικόνων και gif. Υπάρχει ένας Περιηγητής GIF που ανωνυμοποιεί τις αναζητήσεις και τις εικόνες που παρέχονται σε αυτή τη βιβλιοθήκη — απλώς πρέπει να τον χρησιμοποιήσετε.

### Απόδοση

Παρακαλούμε ανοίξτε ένα ticket με ένα παράδειγμα για να αναπαραχθεί το πρόβλημα, συμπεριλαμβανομένης της συσκευής που χρησιμοποιήθηκε, εάν εντοπίσετε προβλήματα απόδοσης. Η απόδοση είναι πρωταρχικής σημασίας σε όλες τις βιβλιοθήκες FastComments.