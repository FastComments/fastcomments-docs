[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Από προεπιλογή, το widget σχολίων FastComments θα ορίζει `gif rating` ως `pg`.

Οι διαθέσιμες επιλογές είναι `g`, `pg`, `pg-13` και `r`.

Αυτό μπορεί να οριστεί στον κώδικα ή μέσω του UI. Στον κώδικα μπορούμε να το κάνουμε ως εξής:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

Στο UI, θα το βρείτε κάτω από `Gif Picker Rating` εφόσον το `Disable Image Uploads?` δεν είναι επιλεγμένο.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]