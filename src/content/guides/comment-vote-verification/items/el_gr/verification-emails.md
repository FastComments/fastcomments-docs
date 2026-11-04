Όταν ένας χρήστης αφήνει ένα σχόλιο ή μια ψήφο, και δεν είναι συνδεδεμένος ή ο λογαριασμός του δεν είναι επαληθευμένος, θα λάβει ένα email που του ζητά να επαληθεύσει αυτή την ενέργεια.

Ωστόσο, καταβάλλουμε κάθε προσπάθεια να μην σπαμάμε τους χρήστες σας με emails, και δεν θα στείλουμε περισσότερα από ένα email επαλήθευσης ανά συνεδρία. Δείτε την ενότητα Sessions για περισσότερες λεπτομέρειες.

Από προεπιλογή, τα emails επαλήθευσης σχολίων μοιάζουν με τα εξής:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Προεπιλεγμένο σώμα email επαλήθευσης που παραθέτει το σχόλιο του Alexander με ένα κουμπί επιβεβαίωσης της δημοσίευσης'; title='Email Επαλήθευσης Σχολίου' app-screenshot-end]

Από προεπιλογή, τα emails επαλήθευσης ψήφων μοιάζουν με τα εξής:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Προεπιλεγμένο email που ζητά από τον Devon να επιβεβαιώσει μια ψήφο, εμφανίζοντας το σχόλιο για το οποίο ψήφισε και ένα κουμπί επιβεβαίωσης'; title='Email Επαλήθευσης Ψήφου' app-screenshot-end]

Από προεπιλογή, το FastComments θα εμφανίζει το λογότυπό του και το όνομά του στο υποσέλιδο αυτών των emails:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Κάτω μέρος ενός email επαλήθευσης που δείχνει το προεπιλεγμένο λογότυπο και όνομα του FastComments στο υποσέλιδο'; title='Υποσέλιδο Email' app-screenshot-end]

Αν βρίσκεστε στα πακέτα Flex ή Pro, [Το όνομα αποστολέα, το email και η επωνυμία μπορούν να προσαρμοστούν](/guide-multiple-sites.html#from-name-email-logo).