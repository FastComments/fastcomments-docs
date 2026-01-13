[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Κατά την αποστολή ειδοποιητικών email, ή κατά την απόδοση σχολίων σε διεπαφές χρήστη όπως η σελίδα μετριασμού, είναι χρήσιμο να μπορείτε να συνδέσετε
το σχόλιο με τη σελίδα στην οποία βρίσκεται.

Αν το URL ID δεν είναι πάντα ένα ID, τότε πρέπει να αποθηκεύσουμε το URL κάπου αλλού. Γι' αυτό υπάρχει η ιδιότητα "url", ορισμένη ως εξής.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Μια συνηθισμένη περίπτωση χρήσης είναι να συσχετίσετε το νήμα σχολίων με έναν αναγνωριστικό, όπως ένα άρθρο, και στη συνέχεια να συνδέσετε πίσω σε μια συγκεκριμένη σελίδα, για παράδειγμα:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

Το URL δεν καθαρίζεται από κοινές παραμέτρους μάρκετινγκ. Από προεπιλογή, όποιο κι αν είναι το τρέχον URL της σελίδας, αυτό είναι το URL που αποθηκεύεται με το σχόλιο.