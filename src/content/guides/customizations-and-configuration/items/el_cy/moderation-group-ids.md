[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Μια λίστα με ids που δημιουργούνται από τη σελίδα [Ομάδες Εποπτείας](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Όταν ορίζεται, τα σχόλια που αφήνονται χρησιμοποιώντας την εν λόγω ρύθμιση θα περιέχουν το ίδιο σύνολο `moderationGroupIds`.

Εάν ένας `Moderator` έχει ορισμένη μία ή περισσότερες [Ομάδες Εποπτείας](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), θα
βλέπει μόνο τα σχόλια στη σελίδα `Moderate Comments` που σχετίζονται με την/τις ομάδα(ές) του.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]