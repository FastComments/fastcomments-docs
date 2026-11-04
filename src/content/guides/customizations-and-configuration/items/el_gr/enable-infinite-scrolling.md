[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το widget του FastComments προσαρμόζει το ύψος του κάθετα ώστε να χωράνε όλα τα ορατά σχόλια. Η σελιδοποίηση επιτυγχάνεται μέσω ενός κουμπιού "View Next" στο τέλος της τρέχουσας σελίδας, καθώς έχουμε διαπιστώσει ότι αυτή η αλληλεπίδραση είναι η πιο ευχάριστη για τους περισσότερους χρήστες.

Ωστόσο, υπάρχουν περιπτώσεις όπου προτιμάται η ατελείωτη κύλιση (infinite scrolling). Για παράδειγμα, χρησιμοποιούμε αυτή τη λειτουργία στο προϊόν μας Stream Chat.

Μπορούμε να κρύψουμε τα κουμπιά "View Next" και να μεταβούμε σε ατελείωτη κύλιση ορίζοντας τη σημαία **enableInfiniteScrolling** σε true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Ενεργοποίηση Ατελείωτης Κύλισης'; code-example-end]

Αυτό απαιτεί επίσης την προσθήκη προσαρμοσμένου CSS. Προσθέστε προσαρμοσμένο CSS για τον selector `.comments` ώστε να ενεργοποιηθεί η κύλιση, για παράδειγμα:

[inline-code-attrs-start title = 'Ενεργοποίηση Ατελείωτης Κύλισης'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Ένα πλήρες λειτουργικό παράδειγμα θα ήταν:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Ενεργοποίηση Ατελείωτης Κύλισης'; code-example-end]

Στο παραπάνω παράδειγμα χρησιμοποιούμε την ιδιότητα `customCSS`, ωστόσο προτείνεται, για λόγους απόδοσης, να χρησιμοποιείται αντ' αυτού το Widget Configuration UI. [Δείτε την τεκμηρίωση Custom CSS.](/guide-customizations-and-configuration.html#custom-css)