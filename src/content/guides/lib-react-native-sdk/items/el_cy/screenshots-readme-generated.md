#### Εμφάνιση: Erebus
![Εμφάνιση: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Εμφάνιση: Default
![Εμφάνιση: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Ενσωματωμένος WYSIWYG επεξεργαστής με υποστήριξη εικόνων!
![Ενσωματωμένος WYSIWYG επεξεργαστής με υποστήριξη εικόνων](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Επεξεργαστής Μορφοποιημένου Κειμένου

Αυτή η βιβλιοθήκη χρησιμοποιεί [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) για επεξεργασία μορφοποιημένου κειμένου, προσφέροντας μια ισχυρή εμπειρία WYSIWYG. Ο ίδιος επεξεργαστής τροφοδοτεί iOS, Android και το web (μέσω `react-native-web`), έτσι ώστε ο συντάκτης να συμπεριφέρεται συνεπώς σε όλες τις πλατφόρμες με μια μόνο υλοποίηση.

`react-native-enriched` απαιτεί το React Native New Architecture (Fabric) στο native, και έναν bundler που επιλύει τις συνθήκες `exports` του πακέτου (Metro με package exports / RN 0.72+). Η υποστήριξη για το web είναι προς το παρόν πειραματική.

### Επιλογές Διαμόρφωσης

Αυτή η βιβλιοθήκη στοχεύει να υποστηρίξει όλες τις επιλογές διαμόρφωσης που ορίζονται στο [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), όπως και η web υλοποίηση.

### Έννοιες FastComments

Οι βασικές έννοιες που πρέπει να γνωρίζετε για να ξεκινήσετε είναι τα `tenantId` και `urlId`. Το `tenantId` είναι το αναγνωριστικό του λογαριασμού σας στο FastComments.com. Το `urlId` είναι το σημείο στο οποίο θα συνδεθούν τα νήματα σχολίων. Αυτό μπορεί να είναι ένα URL σελίδας, ή ένα αναγνωριστικό προϊόντος, ένα αναγνωριστικό άρθρου, κ.λπ.

### Ειδοποιήσεις Χρηστών

Το FastComments υποστηρίζει ειδοποιήσεις για [πολλά σενάρια](https://docs.fastcomments.com/guide-notifications.html). Οι ειδοποιήσεις είναι ρυθμιζόμενες, μπορούν να απενεργοποιηθούν συνολικά ή σε επίπεδο ειδοποίησης/σχολίου, και υποστηρίζουν εγγραφές σε επίπεδο σελίδας ώστε οι χρήστες να μπορούν να εγγραφούν σε νήματα μιας συγκεκριμένης σελίδας ή άρθρου.

Για παράδειγμα, είναι δυνατό να χρησιμοποιήσετε Secure SSO για να αυθεντικοποιήσετε τον χρήστη και στη συνέχεια περιοδικά να ελέγχετε για μη αναγνωσμένες ειδοποιήσεις και να τις στέλνετε στον χρήστη.

Δείτε [το παράδειγμα AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) για το πώς να λαμβάνετε και να μεταφράζετε τις μη αναγνωσμένες ειδοποιήσεις χρηστών.

### Περιηγητής GIF

Εξ ορισμού, δεν είναι ενεργοποιημένη καμία επιλογή εικόνας ή GIF. Δείτε [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) για το πώς να υποστηρίξετε μεταφορτώσεις εικόνων και GIF. Υπάρχει ένας Περιηγητής GIF που ανωνυμοποιεί τις αναζητήσεις και τις εικόνες που παρέχονται σε αυτή τη βιβλιοθήκη — απλώς πρέπει να τον χρησιμοποιήσετε.

### Απόδοση

Παρακαλώ ανοίξτε ένα ticket με ένα παράδειγμα που αναπαράγει το πρόβλημα, συμπεριλαμβανομένης της συσκευής που χρησιμοποιήθηκε, αν εντοπίσετε οποιαδήποτε προβλήματα απόδοσης. Η απόδοση είναι πρωταρχικής σημασίας σε όλες τις βιβλιοθήκες FastComments.