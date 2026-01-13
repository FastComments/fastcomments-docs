[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Όταν ένας χρήστης σχολιάζει με το FastComments για πρώτη φορά θα προσπαθήσουμε να ανακτήσουμε το avatar του από <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Ωστόσο, αν δεν βρούμε avatar, ή ο χρήστης ποτέ δεν ορίσει ένα στον λογαριασμό του, εμφανίζουμε μια στατική προεπιλεγμένη εικόνα avatar.

Για να καθορίσετε τη δική σας στατική εικόνα avatar μπορείτε να χρησιμοποιήσετε την ρύθμιση *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα εξατομίκευσης του widget, δείτε την ενότητα "Προεπιλεγμένο Avatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Σημειώστε ότι ο ορισμός του avatar για έναν συγκεκριμένο χρήστη, όπως με SSO, καλύπτεται σε ξεχωριστή ενότητα.