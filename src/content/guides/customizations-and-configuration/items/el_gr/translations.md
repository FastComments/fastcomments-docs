[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Με το FastComments, όλο το κείμενο στο widget σχολίων είναι προσαρμόσιμο.

Μπορείτε να αντικαταστήσετε ένα μόνο κομμάτι κειμένου, όπως το κουμπί υποβολής, ή όλο το κείμενο σε ολόκληρο το widget σχολίων.

Από προεπιλογή, το κείμενο στο widget σχολίων μεταφράζεται βάσει της τοπικής ρύθμισης του χρήστη. Ωστόσο, μπορούμε να αντικαταστήσουμε το κείμενο, εάν είμαστε σίγουροι ότι η βάση χρηστών μας χρησιμοποιεί την ίδια τοπική γλώσσα, για παράδειγμα:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Προσαρμοσμένο Κείμενο'; code-example-end]

Όλες οι προσαρμόσιμες μεταφράσεις μπορούν να βρεθούν <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">εδώ</a> under the "advanced options" tab.

Ωστόσο, υπάρχει ένας πιο εύκολος τρόπος, μέσω του widget customization UI. Εκεί, μπορούμε απλώς να βρούμε το κείμενο που εμφανίζεται στο commenting widget στην τοπική ρύθμιση EN_US και να ορίσουμε μια αντικατάσταση.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Πίνακας προσαρμοσμένου κειμένου με μια συμβολοσειρά widget επιλεγμένη από το αναπτυσσόμενο μενού και ένα πεδίο κειμένου αντικατάστασης'; title='Προσαρμοσμένο Κείμενο' app-screenshot-end]

Όλες οι παρακάμψεις μεταφράσεων επηρεάζουν επί του παρόντος όλες τις τοπικές ρυθμίσεις.

---