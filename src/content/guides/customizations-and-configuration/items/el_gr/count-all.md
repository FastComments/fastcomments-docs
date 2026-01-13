[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Ο αριθμός σχολίων που εμφανίζεται στην κορυφή του widget σχολίων μπορεί είτε να δείχνει όλα τα σχόλια πρώτου επιπέδου, δηλαδή εκείνες τις απαντήσεις που
είναι απαντήσεις απευθείας στη σελίδα ή το άρθρο, ή μπορεί να είναι μέτρηση **όλων** των εμφωλευμένων σχολίων.

Προεπιλεγμένα, αυτό είναι `true` - είναι μέτρηση του δεύτερου - όλων των σχολίων. Σε παλαιότερες εκδόσεις του widget σχολίων το
προεπιλεγμένο είναι `false`.

Μπορούμε να αλλάξουμε τη συμπεριφορά, ώστε να είναι μέτρηση **όλων** των εμφωλευμένων σχολίων ορίζοντας τη σημαία **countAll** σε true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Εάν θέλαμε ο αριθμός να αντικατοπτρίζει μόνο τα σχόλια πρώτου επιπέδου, ορίζουμε τη σημαία σε false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Αυτό δεν μπορεί αυτή τη στιγμή να προσαρμοστεί χωρίς αλλαγές στον κώδικα.