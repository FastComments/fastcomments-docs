---
Οι χρήστες που έχουν επισημανθεί θα λάβουν ένα email που θα τους ενημερώνει ότι έχουν επισημανθεί ή αναφερθεί σε ένα σχόλιο.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Σώμα email ειδοποίησης που παραθέτει ένα σχόλιο με την αναφορά @winrid σε έντονη γραφή, καθώς και συνδέσμους προβολής και διαγραφής εγγραφής'; title='Ειδοποίηση Αναφοράς Χρήστη' app-screenshot-end]

Η απενεργοποίηση των ειδοποιήσεων θα αποτρέψει αυτά τα email, και παρέχεται μια κεφαλίδα σε κάθε email ώστε οι πελάτες email να μπορούν να επιτρέψουν στον χρήστη να διαγραφεί απρόσκοπτα.

Οι επισημασμένοι χρήστες λαμβάνουν επίσης μια ειδοποίηση εντός της εφαρμογής, ακόμη και όταν δεν έχουν καταχωρημένη διεύθυνση email.

Εάν το σχόλιο περιμένει έγκριση του συντονιστή, το email αναφοράς κρατιέται και αποστέλλεται μόλις εγκριθεί το σχόλιο. Δεν αποστέλλεται εάν το σχόλιο επισημανθεί ως ανεπιθύμητο, διαγραφεί ή δεν εγκριθεί εντός 7 ημερών.

Για να δείτε εάν ένας αναφερθείς χρήστης έλαβε email, ή γιατί δεν το έλαβε, ανοίξτε τα [Αρχεία Σχολίων](/guide-moderation.html#comment-logs) του σχολίου από τη σελίδα Διαχείριση Σχολίων.
---