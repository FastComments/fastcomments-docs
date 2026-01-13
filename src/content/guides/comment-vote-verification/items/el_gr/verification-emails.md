Όταν ένας χρήστης αφήνει ένα σχόλιο ή ψήφο, και δεν είναι συνδεδεμένος, ή ο λογαριασμός του
είναι ανεπαλήθευτος, θα λάβει ένα email που του ζητά να επιβεβαιώσει αυτή την ενέργεια.

Ωστόσο, κάνουμε το καλύτερο δυνατό για να μην βομβαρδίσουμε τους χρήστες σας με μηνύματα ηλεκτρονικού ταχυδρομείου, και δεν θα στείλουμε περισσότερα από ένα
email επαλήθευσης ανά συνεδρία. Δείτε την ενότητα Συνεδρίες για περισσότερες λεπτομέρειες.

Από προεπιλογή, τα email επαλήθευσης σχολίων εμφανίζονται ως εξής:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Comment Verification Email' app-screenshot-end]

Από προεπιλογή, τα email επαλήθευσης ψήφων εμφανίζονται ως εξής:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Vote Verification Email' app-screenshot-end]

Από προεπιλογή, το FastComments θα εμφανίζει το λογότυπο και το όνομά του στο υποσέλιδο αυτών των email:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; title='Email Footer' app-screenshot-end]

Εάν βρίσκεστε στα επίπεδα Flex ή Pro, [Το όνομα αποστολέα, το email και η εταιρική ταυτότητα μπορούν να προσαρμοστούν](/guide-multiple-sites.html#from-name-email-logo).