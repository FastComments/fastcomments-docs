[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Ο αριθμός σχολίων που εμφανίζεται στην κορυφή του widget σχολίων μπορεί να προσαρμοστεί.

Αυτό μπορεί να αντικατασταθεί με οποιαδήποτε συμβολοσειρά, και η τιμή **[count]** θα αντικατασταθεί με την τιμή του αριθμού, τοπικοποιημένη για τον χρήστη.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Αυτό μπορεί να προσαρμοστεί χωρίς κώδικα, στη σελίδα προσαρμογής του widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Πεδίο κειμένου αριθμού σχολίων στη σελίδα προσαρμογής widget, όπου το [count] αντικαθίσταται με το ζωντανό σύνολο'; title='Προσαρμογή του κειμένου αριθμού σχολίων' app-screenshot-end]