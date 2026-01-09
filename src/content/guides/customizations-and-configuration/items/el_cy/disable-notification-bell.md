---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Από προεπιλογή, το FastComments θα εμφανίζει ένα κουδούνι ειδοποιήσεων στην επάνω δεξιά γωνία της περιοχής σχολίων.

Αυτό το κουδούνι θα γίνει κόκκινο και θα εμφανίσει τον αριθμό των ειδοποιήσεων που έχει ο χρήστης. Μερικά παραδείγματα ειδοποιήσεων είναι:

- Κάποιος χρήστης σας απάντησε.
- Κάποιος χρήστης απάντησε σε ένα νήμα όπου σχολιάσατε.
- Κάποιος χρήστης ψήφισε θετικά το σχόλιό σας.
- Κάποιος χρήστης απάντησε σε μια σελίδα στην οποία έχετε εγγραφεί.

Το κουδούνι ειδοποιήσεων παρέχει επίσης έναν μηχανισμό για εγγραφή σε ολόκληρη σελίδα.

Ωστόσο, μπορούμε να απενεργοποιήσουμε εντελώς το κουδούνι ειδοποιήσεων:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Αυτό μπορεί επίσης να γίνει χωρίς κώδικα. Στη σελίδα προσαρμογής του widget, δείτε την ενότητα "Απενεργοποίηση κουδουνιού ειδοποιήσεων".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---