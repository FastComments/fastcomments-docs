#### Skin: Erebus
![Skin: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Skin: Default
![Skin: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Ένθετος εγγενής επεξεργαστής WYSIWYG με υποστήριξη εικόνων!
![Native WYSIWYG Editor with Image Support](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Rich Text Editor

Αυτή η βιβλιοθήκη χρησιμοποιεί τον επεξεργαστή 10tap για λειτουργικότητα επεξεργασίας πλούσιου κειμένου, προσφέροντας μια ισχυρή εμπειρία επεξεργασίας WYSIWYG.

### Configuration Options

Αυτή η βιβλιοθήκη στοχεύει να υποστηρίζει όλες τις επιλογές ρύθμισης που ορίζονται στο [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), όπως και η υλοποίηση για τον ιστό.

### FastComments Concepts

Οι κύριες έννοιες που πρέπει να γνωρίζετε για να ξεκινήσετε είναι οι `tenantId` και `urlId`. Το `tenantId` είναι ο αναγνωριστικός λογαριασμός σας στο FastComments.com. Το `urlId` είναι αυτό στο οποίο θα συνδεθούν τα νήματα σχολίων. Αυτό μπορεί να είναι μια διεύθυνση σελίδας, ένα αναγνωριστικό προϊόντος, ένα αναγνωριστικό άρθρου κ.λπ.

### User Notifications

Το FastComments υποστηρίζει ειδοποιήσεις για [πολλά σενάρια](https://docs.fastcomments.com/guide-notifications.html). Οι ειδοποιήσεις είναι παραμετροποιήσιμες, μπορούν να απενεργοποιηθούν παγκοσμίως ή σε επίπεδο ειδοποίησης/σχολίου, και υποστηρίζει συνδρομές σε επίπεδο σελίδας ώστε οι χρήστες να μπορούν να εγγράφονται σε νήματα συγκεκριμένης σελίδας ή άρθρου.

Για παράδειγμα, είναι δυνατόν να χρησιμοποιηθεί Secure SSO για να αυθεντικοποιηθεί ο χρήστης και στη συνέχεια να γίνει περιοδική επερώτηση για μη αναγνωσμένες ειδοποιήσεις και να προωθηθούν στον χρήστη.

Δείτε το [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) για το πώς να λάβετε και να μεταφράσετε μη αναγνωσμένες ειδοποιήσεις χρήστη.

### Gif Browser

Εξ ορισμού, δεν είναι ενεργοποιημένη καμία επιλογή εικόνας ή gif. Δείτε [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) για το πώς να υποστηρίξετε μεταφορτώσεις εικόνων και gifs. Υπάρχει ένας Περιηγητής GIF που αποανωνυμοποιεί τις αναζητήσεις και τις εικόνες που παρέχονται σε αυτή τη βιβλιοθήκη — απλά πρέπει να τον χρησιμοποιήσετε.

### Performance

Παρακαλούμε ανοίξτε ένα ticket με ένα παράδειγμα αναπαραγωγής, συμπεριλαμβανομένης της συσκευής που χρησιμοποιήθηκε, αν εντοπίσετε οποιοδήποτε πρόβλημα απόδοσης. Η απόδοση είναι πρωταρχικής σημασίας σε όλες τις βιβλιοθήκες FastComments.