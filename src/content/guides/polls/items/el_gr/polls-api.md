---
Οι δημοσκοπήσεις μπορούν επίσης να διαχειρίζονται από τον δικό σας κώδικα μέσω του REST API, για παράδειγμα για να προσθέσετε μια δημοσκόπηση σε ένα σχόλιο από το backend σας, να κλείσετε μια δημοσκόπηση ή να εξάγετε τις ψήφους:

- [Poll Structure](/guide-api.html#poll-structure) περιγράφει το αντικείμενο της δημοσκόπησης και τα όριά του.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), and [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` διαβάζει, αντικαθιστά, επεξεργάζεται και αφαιρεί τη δημοσκόπηση σε ένα σχόλιο.
- [PollVote Structure](/guide-api.html#poll-vote-structure) περιγράφει μια μεμονωμένη ψήφο.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), and [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` καταγράφει, καταχωρεί και αφαιρεί ψήφους.

Μια δημοσκόπηση μπορεί επίσης να δημιουργηθεί μαζί με το σχόλιό της, περνώντας το `poll` στο `POST /api/v1/comments`.

Το API ακολουθεί τις ίδιες ρυθμίσεις με το widget: οι δημοσκοπήσεις πρέπει να είναι ενεργοποιημένες, και οι ψήφοι χωρίς συνδεδεμένο χρήστη απορρίπτονται εκτός εάν επιτρέπονται ανώνυμες ψήφοι δημοσκόπησης. Οι μεμονωμένες ψήφοι σε ανώνυμες δημοσκοπήσεις δεν μπορούν να διαβαστούν μέσω του API.

---