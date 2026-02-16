#### Θέμα: Erebus
![Θέμα: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Θέμα: Default
![Θέμα: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Ενσωματωμένος επεξεργαστής WYSIWYG με υποστήριξη εικόνων!
![Ενσωματωμένος επεξεργαστής WYSIWYG με υποστήριξη εικόνων](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Επεξεργαστής Πλούσιου Κειμένου

Αυτή η βιβλιοθήκη χρησιμοποιεί τον επεξεργαστή 10tap για λειτουργικότητα επεξεργασίας πλούσιου κειμένου, ο οποίος παρέχει μια ισχυρή εμπειρία επεξεργασίας WYSIWYG.

### Επιλογές Διαμόρφωσης

Αυτή η βιβλιοθήκη στοχεύει να υποστηρίζει όλες τις επιλογές διαμόρφωσης που ορίζονται στο [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), όπως και η υλοποίηση για τον ιστό.

### Έννοιες FastComments

Οι κύριες έννοιες που πρέπει να γνωρίζετε για να ξεκινήσετε είναι `tenantId` και `urlId`. Το `tenantId` είναι ο αναγνωριστικός λογαριασμός σας στο FastComments.com. Το `urlId` είναι όπου θα συνδεθούν τα θέματα σχολίων. Αυτό μπορεί να είναι ένα URL σελίδας, ή ένα αναγνωριστικό προϊόντος, ένα αναγνωριστικό άρθρου, κ.λπ.

### Ειδοποιήσεις Χρήστη

Το FastComments υποστηρίζει ειδοποιήσεις για [πολλές περιπτώσεις](https://docs.fastcomments.com/guide-notifications.html). Οι ειδοποιήσεις είναι παραμετροποιήσιμες, μπορούν να απενεργοποιηθούν παγκοσμίως ή σε επίπεδο ειδοποίησης/σχολίου, και υποστηρίζουν εγγραφές σε επίπεδο σελίδας ώστε οι χρήστες να μπορούν να εγγράφονται σε νήματα μιας συγκεκριμένης σελίδας ή άρθρου.

Για παράδειγμα, είναι δυνατό να χρησιμοποιηθεί Secure SSO για να ελέγξετε τον χρήστη και στη συνέχεια περιοδικά να ελέγχετε για μη αναγνωσμένες ειδοποιήσεις και να τις προωθείτε στον χρήστη.

Δείτε το παράδειγμα [AppNotificationsSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) για το πώς να λαμβάνετε και να μεταφράζετε τις μη αναγνωσμένες ειδοποιήσεις χρήστη.

### Περιηγητής GIF

Από προεπιλογή, δεν είναι ενεργοποιημένη η επιλογή εικόνας ή gif. Δείτε [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) για το πώς να υποστηρίξετε ανεβάσματα εικόνων και gif. Σε αυτή τη βιβλιοθήκη υπάρχει ένας Περιηγητής GIF που ανωνυμοποιεί τις αναζητήσεις και τις εικόνες που παρέχονται, απλώς πρέπει να τον χρησιμοποιήσετε.

### Απόδοση

Παρακαλώ ανοίξτε ένα ticket με ένα παράδειγμα αναπαραγωγής, συμπεριλαμβανομένης της συσκευής που χρησιμοποιήθηκε, εάν εντοπίσετε οποιαδήποτε προβλήματα απόδοσης. Η απόδοση είναι πρωταρχικής σημασίας σε όλες τις βιβλιοθήκες FastComments.