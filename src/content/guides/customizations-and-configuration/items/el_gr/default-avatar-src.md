[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Όταν ένας χρήστης σχολιάζει με το FastComments για πρώτη φορά θα προσπαθήσουμε να ανακτήσουμε την εικόνα προφίλ τους από <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Ωστόσο, αν δεν βρούμε εικόνα προφίλ, ή ο χρήστης δεν ορίσει ποτέ κάποια στον λογαριασμό του, εμφανίζουμε μια στατική προεπιλεγμένη εικόνα προφίλ.

Για να καθορίσετε τη δική σας στατική εικόνα προφίλ μπορούμε να χρησιμοποιήσουμε τη ρύθμιση *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Προεπιλεγμένη εικόνα προφίλ".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Σημειώστε ότι ο καθορισμός της εικόνας προφίλ για έναν συγκεκριμένο χρήστη, όπως με SSO, καλύπτεται σε ξεχωριστή ενότητα.