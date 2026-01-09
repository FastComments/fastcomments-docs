[related-parameter-start name = 'moderationGroupIds'; type = 'Array<string>'; related-parameter-end]

Μια λίστα με αναγνωριστικά που δημιουργούνται από τη σελίδα [Ομάδες Επιτήρησης](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

Όταν καθορίζεται, τα σχόλια που αφήνονται χρησιμοποιώντας την καθορισμένη διαμόρφωση θα περιέχουν το ίδιο σύνολο `moderationGroupIds`.

Εάν ένας `Moderator` έχει ορίσει μία ή περισσότερες [Ομάδες Επιτήρησης](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups),
θα βλέπουν μόνο τα σχόλια στη σελίδα `Moderate Comments` που σχετίζονται με την ομάδα/τις ομάδες τους.

[code-example-start config = {moderationGroupIds: ['mxZAhjzdb', 'FT19nXbqA']}; linesToHighlight = [6, 7, 8, 9]; title = 'Specify Moderation Groups'; code-example-end]

---