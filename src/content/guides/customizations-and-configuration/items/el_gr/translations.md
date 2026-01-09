[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Με το FastComments, όλο το κείμενο στο widget σχολιασμού είναι προσαρμόσιμο.

Μπορείτε να αντικαταστήσετε ένα μόνο κομμάτι κειμένου, όπως το κουμπί υποβολής, ή όλο το κείμενο σε ολόκληρο το widget σχολιασμού.

Εξ ορισμού, το κείμενο στο widget σχολιασμού μεταφράζεται βάσει της τοπικής ρύθμισης (locale) του χρήστη. Ωστόσο, μπορούμε να αντικαταστήσουμε το κείμενο, εάν είμαστε βέβαιοι
ότι η βάση χρηστών μας χρησιμοποιεί την ίδια τοπική ρύθμιση/γλώσσα, για παράδειγμα:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Όλες οι προσαρμόσιμες μεταφράσεις μπορείτε να τις βρείτε <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">εδώ</a> στην καρτέλα «Σύνθετες επιλογές».

Ωστόσο, υπάρχει ένας πιο εύκολος τρόπος, μέσω του UI προσαρμογής του widget. Εκεί, μπορούμε απλά να βρούμε το κείμενο που εμφανίζεται στο widget σχολιασμών στην τοπική ρύθμιση EN_US και να καθορίσουμε μια αντικατάσταση.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Όλες οι αντικαταστάσεις μεταφράσεων επηρεάζουν επί του παρόντος όλες τις τοπικές ρυθμίσεις.