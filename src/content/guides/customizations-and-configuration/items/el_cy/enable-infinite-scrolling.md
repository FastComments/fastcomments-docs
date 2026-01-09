[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το widget FastComments θα αλλάζει το ύψος του κάθετα ώστε να χωράει όλα τα ορατά σχόλια. Η σελιδοποίηση επιτυγχάνεται μέσω ενός κουμπιού "Προβολή Επόμενων" στο τέλος της τρέχουσας σελίδας, καθώς διαπιστώσαμε ότι αυτή η αλληλεπίδραση είναι η πιο άνετη για τους περισσότερους χρήστες.

Ωστόσο, υπάρχουν περιπτώσεις όπου προτιμάται η άπειρη κύλιση. Για παράδειγμα, χρησιμοποιούμε αυτή τη δυνατότητα στο προϊόν Stream Chat.

Μπορούμε να αποκρύψουμε τα κουμπιά "Προβολή Επόμενων" και να μεταβούμε στην άπειρη κύλιση ρυθμίζοντας τη σημαία **enableInfiniteScrolling** σε true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Αυτό απαιτεί επίσης την προσθήκη προσαρμοσμένου CSS. Προσθέστε προσαρμοσμένο CSS για τον επιλεκτή `.comments` για να ενεργοποιήσετε την κύλιση, για παράδειγμα:

[inline-code-attrs-start title = 'Ενεργοποίηση Άπειρης Κύλισης'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Ένα πλήρες λειτουργικό παράδειγμα θα ήταν:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

Στο παραπάνω παράδειγμα χρησιμοποιούμε την ιδιότητα `customCSS`, ωστόσο συνιστάται να χρησιμοποιηθεί η διεπαφή διαμόρφωσης του widget (Widget Configuration UI) για λόγους απόδοσης. [Δείτε την τεκμηρίωση για το Custom CSS.](/guide-customizations-and-configuration.html#custom-css)