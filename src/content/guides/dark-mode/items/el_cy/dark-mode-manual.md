### Για προγραμματιστές - Αναγκαστική απενεργοποίηση σκοτεινής λειτουργίας

Η αναγκαστική απενεργοποίηση της σκοτεινής λειτουργίας μπορεί να γίνει περνώντας το `hasDarkBackground` ως `false` στη διαμόρφωση του widget. Αυτό λειτουργεί για τις βιβλιοθήκες VanillaJS, Angular, React, Vue και React Native.

Κάθε βιβλιοθήκη έχει έναν φάκελο `examples` στο [GitHub](https://github.com/fastComments/) που περιέχει παραδείγματα για το πώς να χρησιμοποιήσετε τη σκοτεινή λειτουργία.

### Αναγκαστική ενεργοποίηση σκοτεινής λειτουργίας

Μπορούμε να αναγκάσουμε τη σκοτεινή λειτουργία να είναι πάντα ενεργοποιημένη ορίζοντας το `hasDarkBackground` σε `true`.

Μπορούμε επίσης να το κάνουμε αυτό μέσω του UI προσαρμογής Widget [εδώ](https://fastcomments.com/auth/my-account/customize-widget).

Κάτω από το `Base Theme` απλά επιλέξτε `Force Dark Mode`.

### VanillaJS Widget - Ενημέρωση σκοτεινής λειτουργίας

Ο ευκολότερος τρόπος για να ενημερώσετε τη σκοτεινή λειτουργία είναι να περάσετε από όλες τις περιπτώσεις του widget στη σελίδα και να ενημερώσετε τη διαμόρφωσή τους:

[inline-code-attrs-start title = 'VanillaJS - Dark Mode Toggle'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    let isDarkMode = somehowDetermineIfDarkModeEnabled();
    for (const instanceWrapped of window.fcUIInstances) {
        if (instanceWrapped.targetElement) {
            const config = instanceWrapped.config;
            config.hasDarkBackground = isDarkMode;
            instanceWrapped.instance.update(config)
        }
    }
[inline-code-end]
