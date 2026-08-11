[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Από προεπιλογή, το FastComments comment widget θα ορίσει μια `gif rating` τιμής `pg`.

Διαθέσιμες επιλογές είναι `g`, `pg`, `pg-13` και `r`.

Αυτό μπορεί να οριστεί στον κώδικα ή μέσω του UI. Στον κώδικα μπορούμε να το κάνουμε ως εξής:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Ορισμός Βαθμολογίας GIF'; code-example-end]

Στο UI, θα το βρείτε κάτω από `Gif Picker Rating` εφόσον η επιλογή `Disable Image Uploads?` δεν είναι επιλεγμένη.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Αναπτυσσόμενο μενού Gif Picker Rating στη σελίδα προσαρμογής widget που προσφέρει g, pg, pg-13 και r'; title='Ορισμός της Βαθμολογίας GIF' app-screenshot-end]